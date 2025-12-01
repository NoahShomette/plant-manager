//! Stores a local copy of plants for lowered network usage and faster responses

use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use gloo_net::http::Request;
use leptos::{
    prelude::{Signal, Write, WriteSignal},
    reactive::spawn_local,
    server::codee::string::JsonSerdeCodec,
};

use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use shared::events::{events_http::NewEvent, EventData, EventDataKind, EventInstance, EventType};
use uuid::Uuid;

use crate::{
    data_storage::events::event_storage::{
        EventInstanceStorageComponent, EventStorageContext, PlantEvents,
    },
    default_http_request,
};

use leptos::prelude::*;

pub mod event_storage;

#[component]
pub fn EventStorageComponent(children: Children) -> impl IntoView {
    let (pv, pv_set) = signal(LastRequest::default());

    provide_context(LastRequestContext {
        get: pv,
        write: pv_set,
    });

    let (pl_state, pl_set_state) = signal(EventTypeList::default());

    provide_context(EventListContext {
        get_event_list: pl_state,
        write_plant_list: pl_set_state,
    });
    let plant_list_context: EventListContext = expect_context::<EventListContext>();

    let pv_context: LastRequestContext = expect_context::<LastRequestContext>();

    Effect::new(move |_| {
        spawn_local(get_event_type_list(
            pv_context.get.get_untracked(),
            pv_context.write,
            plant_list_context.write_plant_list,
        ))
    });

    set_interval(
        move || {
            Effect::new(move |_| {
                spawn_local(get_event_type_list(
                    pv_context.get.get_untracked(),
                    pv_context.write,
                    plant_list_context.write_plant_list,
                ))
            });
        },
        Duration::from_secs(60),
    );

    view! { <EventInstanceStorageComponent>{children()}</EventInstanceStorageComponent> }
}

#[derive(Clone, PartialEq)]
pub struct EventListContext {
    pub get_event_list: ReadSignal<EventTypeList>,
    pub write_plant_list: WriteSignal<EventTypeList>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct EventTypeList(pub Vec<EventType>);

#[derive(Clone, PartialEq)]
pub struct LastRequestContext {
    pub get: ReadSignal<LastRequest>,
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

async fn get_event_type_list(
    last_requested: LastRequest,
    last_requested_write: WriteSignal<LastRequest>,
    plant_list_write: WriteSignal<EventTypeList>,
) {
    let request = Request::get(&format!(
        "http://localhost:8080/events/get-types/{}",
        last_requested.0.and_utc().timestamp()
    ));
    let request = default_http_request(request);

    let Some(response) = request.send().await.map_err(|e| log::error!("{e}")).ok() else {
        //TODO: Background Error message logging
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        //TODO: Background Error message logging
        return;
    };

    let Ok(response) = serde_json::de::from_str::<Vec<EventType>>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };

    // plant_storage.plants.insert(new_plant.id, new_plant.clone());

    // *plant_storage_write.write() = plant_storage;

    // *submit_response_2.write() = format!("{:?}", new_plant);

    // TODO: Queue a resync of the PlantStorage now. If we've deleted plants then we want to remove them asap and if we've spawned new plants then we want to pull their demographics if we dont have that already
    plant_list_write.write().0 = response;
    last_requested_write.write().0 = Utc::now().naive_utc();
}

pub fn new_event_action() -> Action<(NewEvent), ()> {
    Action::new_local(|input: &NewEvent| {
        new_event(input.clone())
    })
}

async fn new_event(new_event: NewEvent) {
    let request = Request::post(&format!("http://localhost:8080/events/new"));
    let request = default_http_request(request);

    let Some(request_with_json) = request
        .json(&new_event)
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        return;
    };

    let Some(response) = request_with_json
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        return;
    };

    let Ok(response) = serde_json::de::from_str::<EventInstance>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };

}
