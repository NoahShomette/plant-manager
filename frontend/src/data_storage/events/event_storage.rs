//! Stores a local copy of plants for lowered network usage and faster responses

use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::{
    prelude::{Signal, Write, WriteSignal},
    reactive::spawn_local,
    server::codee::string::JsonSerdeCodec,
};

use leptos_use::storage::use_session_storage;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use shared::events::{
    events_http::{GetEvent, GetEventResponse, NewEvent},
    EventInstance, EventType,
};
use uuid::Uuid;

use crate::FrontEndState;

use leptos::prelude::*;

#[component]
pub fn EventInstanceStorageComponent(children: Children) -> impl IntoView {
    let (pl_state, pl_set_state, _) =
        use_session_storage::<EventStorage, JsonSerdeCodec>("event-storage");

    provide_context(EventStorageContext {
        get_event_storage: pl_state,
        write_event_storage: pl_set_state,
    });
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let plant_list_context: EventStorageContext = expect_context::<EventStorageContext>();

    view! { {children()} }
}

#[derive(Clone, PartialEq)]
pub struct EventStorageContext {
    pub get_event_storage: Signal<EventStorage>,
    pub write_event_storage: WriteSignal<EventStorage>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct EventStorage {
    pub events: HashMap<Uuid, EventInstance>,
    /// An index from plants id to the events id
    pub plants_index: HashMap<Uuid, Vec<Uuid>>,
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

pub fn submit_event(request_details: NewEvent, reqwest_client: FrontEndState) {
    spawn_local(submit_event_request(reqwest_client, request_details));
}

pub fn request_events_resource(request_details: GetEvent) -> LocalResource<Vec<EventInstance>> {
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let event_storage_context: EventStorageContext = expect_context::<EventStorageContext>();
    LocalResource::new(move || {
        request_events(
            reqwest_client.get(),
            request_details.clone(),
            event_storage_context.write_event_storage,
        )
    })
}

async fn request_events(
    reqwest_client: FrontEndState,
    request_details: GetEvent,
    plant_storage_write: WriteSignal<EventStorage>,
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
    let mut write = plant_storage_write.write();

    for event in response.iter() {
        write
            .plants_index
            .entry(event.plant_id)
            .and_modify(|entry| {
                if !entry.contains(&event.id) {
                    entry.push(event.id);
                }
            })
            .or_insert(vec![event.id]);
        write.events.insert(event.id, event.clone());
    }
    response
}

async fn submit_event_request(reqwest_client: FrontEndState, request_details: NewEvent) {
    let Some(response) = reqwest_client
        .client
        .post("http://localhost:8080/events/new")
        .json(&request_details)
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        //TODO: Background Error message logging
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        //TODO: Background Error message logging
        return;
    };
}
