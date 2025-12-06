//! Stores a local copy of plants for lowered network usage and faster responses

use std::{
    collections::{BTreeMap, HashMap},
    ops::Bound,
};

use chrono::{DateTime, NaiveDateTime};
use leptos::{
    leptos_dom::logging::console_log,
    prelude::{Signal, Write, WriteSignal},
};

use serde::{Deserialize, Serialize};
use shared::events::{
    events_http::{GetEvent, GetEventType},
    EventInstance,
};
use uuid::Uuid;

use crate::{
    data_storage::{DirtyManager, DirtyManagerContext},
    server_helpers::post_request,
};

use leptos::prelude::*;

#[component]
pub fn EventInstanceStorageComponent(children: Children) -> impl IntoView {
    let (pl_state, pl_set_state) = signal(EventStorage::new());

    provide_context(EventStorageContext {
        get_event_storage: pl_state,
        write_event_storage: pl_set_state,
    });

    view! { {children()} }
}

#[derive(Clone, PartialEq, Copy)]
pub struct EventStorageContext {
    pub get_event_storage: ReadSignal<EventStorage>,
    pub write_event_storage: WriteSignal<EventStorage>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct EventStorage {
    /// An index from plants id to that plants events
    pub plants_index: HashMap<Uuid, PlantEvents>,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            plants_index: HashMap::new(),
        }
    }
    pub fn get_events(
        &self,
        plant_id: Uuid,
        get_event: GetEventType,
        event_id: Uuid,
    ) -> Option<Vec<EventInstance>> {
        return match self.plants_index.get(&plant_id) {
            Some(plants) => plants.get_events(get_event, event_id),
            None => None,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct PlantEvents {
    pub earliest_event: Option<NaiveDateTime>,
    /// Event type id and a map of those events
    pub events: HashMap<Uuid, BTreeMap<NaiveDateTime, EventInstance>>,
}

impl PlantEvents {
    pub fn get_events(
        &self,
        get_event: GetEventType,
        event_id: Uuid,
    ) -> Option<Vec<EventInstance>> {
        let Some(events) = self.events.get(&event_id) else {
            return None;
        };
        match get_event {
            GetEventType::Span(naive_date_time, naive_date_time1) => Some(
                events
                    .range((
                        Bound::Included(naive_date_time),
                        Bound::Included(naive_date_time1),
                    ))
                    .map(|event| event.1.clone())
                    .collect(),
            ),
            GetEventType::LastNth(n) => Some(
                events
                    .iter()
                    .map(|event| event.1.clone())
                    .rev()
                    .take(n as usize)
                    .collect(),
            ),
            GetEventType::All => Some(events.iter().map(|event| event.1.clone()).collect()),
        }
    }

    pub fn new_from_events(events: Vec<EventInstance>) -> Self {
        let mut new = Self::default();
        new.add_new_events(events);
        new
    }

    pub fn add_new_events(&mut self, events: Vec<EventInstance>) {
        for event in events {
            match self.earliest_event {
                Some(date) => {
                    if event.event_date.and_utc().timestamp() < date.and_utc().timestamp() {
                        self.earliest_event = Some(event.event_date)
                    }
                }
                None => self.earliest_event = Some(event.event_date),
            }

            self.events
                .entry(event.event_type_id)
                .and_modify(|event_list| {
                    event_list.insert(event.event_date, event.clone());
                })
                .or_insert_with(|| {
                    let mut map = BTreeMap::new();
                    map.insert(event.event_date, event);
                    map
                });
        }
    }

    pub fn clear(&mut self) {
        self.earliest_event = None;
        self.events = HashMap::new();
    }

    /// Conducts a simple check to see if the local plant events are able to fulfill the events request
    ///
    /// Note that ALL will always return false
    pub fn can_fulfill_request(&self, request: &GetEvent) -> bool {
        match request.request_details {
            GetEventType::Span(naive_date_time, _naive_date_time1) => {
                let Some(earliest_event) = self.earliest_event else {
                    return false;
                };
                naive_date_time >= earliest_event
            }
            GetEventType::LastNth(nth) => {
                self.events.get(&request.event_type).iter().count() >= nth as usize
            }
            GetEventType::All => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LastRequestContext {
    pub get: Signal<LastRequest>,
    pub write: WriteSignal<LastRequest>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LastRequest(pub NaiveDateTime);

impl Default for LastRequest {
    fn default() -> Self {
        Self(DateTime::UNIX_EPOCH.naive_utc())
    }
}

pub fn request_events_resource(
    request_details: RwSignal<GetEvent>,
) -> LocalResource<Vec<EventInstance>> {
    let dirty_mangaer = expect_context::<DirtyManagerContext>();
    let event_storage_context = expect_context::<EventStorageContext>();
    LocalResource::new(move || {
        request_events(
            dirty_mangaer.write,
            dirty_mangaer.get.get(),
            request_details.get(),
            event_storage_context,
        )
    })
}

async fn request_events(
    dirty_manager: WriteSignal<DirtyManager>,
    dirty_manager_read: DirtyManager,
    request_details: GetEvent,
    event_storage_context: EventStorageContext,
) -> Vec<EventInstance> {
    // See if we have any saved events for the requested plant

    let read_event_storage = match event_storage_context.get_event_storage.try_get_untracked() {
        Some(data) => data,
        None => return vec![],
    };

    match read_event_storage
        .plants_index
        .get(&request_details.plant_id)
    {
        Some(plant_events) => {
            // If we do have a saved events then lets request the dirty manager and see if it needs updating for our requested events
            if let Some((events, _earliest_dirty_event)) =
                dirty_manager_read.events.get(&request_details.plant_id)
            {
                console_log(&format!("Local events found"));

                if events.contains(&request_details.event_type) {
                    console_log(&format!(
                        "Dirty Manager contains event_type: {}",
                        request_details.event_type
                    ));

                    let new_events = request_events_http(
                        request_details.clone(),
                        event_storage_context.write_event_storage,
                    )
                    .await;

                    let mut dirty_manager = match dirty_manager.try_write_untracked() {
                        Some(data) => data,
                        None => return vec![],
                    };
                    dirty_manager.clean_event(request_details.plant_id, request_details.event_type);
                    return new_events;
                }
            }
            // If theres no dirty cache then check if we have this event saved

            return match plant_events.can_fulfill_request(&request_details) {
                true => match plant_events.get_events(
                    request_details.request_details.clone(),
                    request_details.event_type,
                ) {
                    Some(events) => events,
                    None => {
                        vec![]
                    }
                },
                false => {
                    request_events_http(
                        request_details.clone(),
                        event_storage_context.write_event_storage,
                    )
                    .await
                }
            };

            // We have saved events so check the cache and see if its dirty
            // If it is dirty send the request for new events after clearing our current events
            // If its not dirty see if the requested events fall within our cache. If they do then just return those evennts. If not send a request to cover the gap
        }
        None => {
            console_log(&format!("No Event data saved"));
            // There are no saved events so we need to request new ones and return those
            return request_events_http(request_details, event_storage_context.write_event_storage)
                .await;
        }
    }
}

async fn request_events_http(
    request_details: GetEvent,
    event_storage: WriteSignal<EventStorage>,
) -> Vec<EventInstance> {
    let request = post_request("/events/get-events");

    let Some(request_with_json) = request
        .json(&request_details)
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        return vec![];
    };

    let Some(response) = request_with_json
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        //TODO: Background Error message logging
        return vec![];
    };
    let Some(body_text) = response.text().await.ok() else {
        //TODO: Background Error message logging
        return vec![];
    };

    let Ok(response) = serde_json::de::from_str::<Vec<EventInstance>>(&body_text) else {
        //TODO: Background Error message logging
        return vec![];
    };

    let mut write = match event_storage.try_write() {
        Some(data) => data,
        None => return vec![],
    };

    write
        .plants_index
        .entry(request_details.plant_id)
        .and_modify(|entry| {
            entry.add_new_events(response.clone());
        })
        .or_insert(PlantEvents::new_from_events(response.clone()));

    response
}
