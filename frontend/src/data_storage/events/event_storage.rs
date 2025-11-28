//! Stores a local copy of plants for lowered network usage and faster responses

use std::{
    collections::{BTreeMap, HashMap},
    ops::Bound,
    time::Duration,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::{
    prelude::{Signal, Write, WriteSignal},
    reactive::spawn_local,
    server::codee::string::JsonSerdeCodec,
};

use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use shared::{
    events::{
        self,
        events_http::{GetEvent, GetEventResponse, GetEventType, NewEvent},
        EventInstance, EventType,
    },
    DirtyCache,
};
use uuid::Uuid;

use crate::{
    data_storage::{DirtyManager, DirtyManagerContext},
    FrontEndState,
};

use leptos::prelude::*;

#[component]
pub fn EventInstanceStorageComponent(children: Children) -> impl IntoView {
    /* For some reason this crashes with a use_signal error
        let (pl_state, pl_set_state, _) =
            use_session_storage::<EventStorage, JsonSerdeCodec>("event-storage");
    */
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
        let mut new_earliest: Option<NaiveDateTime> = None;
        for event in events {
            match self.earliest_event {
                Some(date) => {
                    if event.event_date.and_utc().timestamp() < date.and_utc().timestamp() {
                        new_earliest = Some(event.event_date)
                    }
                }
                None => new_earliest = Some(event.event_date),
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

        if let Some(date) = new_earliest {
            self.earliest_event = Some(date);
        }
    }

    pub fn clear(&mut self) {
        self.earliest_event = None;
        self.events = HashMap::new();
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
    request_details: GetEvent,
    plant_events: RwSignal<PlantEvents>,
) -> LocalResource<Vec<EventInstance>> {
    let dirty_mangaer = expect_context::<DirtyManagerContext>();
    let reqwest_client = expect_context::<Store<FrontEndState>>();
    //let event_storage_context = expect_context::<EventStorageContext>();
    LocalResource::new(move || {
        request_events(
            dirty_mangaer.write,
            reqwest_client.get(),
            request_details.clone(),
            plant_events,
        )
    })
}

async fn request_events(
    dirty_manager: WriteSignal<DirtyManager>,
    reqwest_client: FrontEndState,
    request_details: GetEvent,
    local_events: RwSignal<PlantEvents>,
) -> Vec<EventInstance> {
    // See if we have any saved events for the requested plant

    let read_event_storage = match local_events.try_get() {
        Some(data) => data,
        None => return vec![],
    };

    match read_event_storage.events.is_empty() {
        false => {
            // If we do have a saved events then lets request the dirty manager and see if it needs updating for our requested events
            let mut dirty_manager = match dirty_manager.try_write() {
                Some(data) => data,
                None => return vec![],
            };

            if let Some((events, _earliest_dirty_event)) =
                dirty_manager.events.get_mut(&request_details.plant_id)
            {
                if events.contains(&request_details.event_type) {
                    let new_events = request_events_http(
                        reqwest_client,
                        request_details.clone(),
                        local_events.write_only(),
                    )
                    .await;
                    dirty_manager.clean_event(request_details.plant_id, request_details.event_type);
                    return new_events;
                }
            }
            return match read_event_storage
                .get_events(request_details.request_details, request_details.event_type)
            {
                Some(events) => events,
                None => vec![],
            };
            // We have saved events so check the cache and see if its dirty
            // If it is dirty send the request for new events after clearing our current events
            // If its not dirty see if the requested events fall within our cache. If they do then just return those evennts. If not send a request to cover the gap
        }
        true => {
            // There are no saved events so we need to request new ones and return those
            return request_events_http(reqwest_client, request_details, local_events.write_only())
                .await;
        }
    }
}

async fn request_events_http(
    reqwest_client: FrontEndState,
    request_details: GetEvent,
    event_storage: WriteSignal<PlantEvents>,
) -> Vec<EventInstance> {
    let Some(response) = reqwest_client
        .client
        .post("http://localhost:8080/events/get-events")
        .json(&request_details)
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

    write.add_new_events(response.clone());

    response
}
