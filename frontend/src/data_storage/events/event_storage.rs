//! Stores a local copy of plants for lowered network usage and faster responses


use leptos::{
};

use shared::events::{
    events_http::{GetEvent},
    EventInstance,
};

use crate::{
    server_helpers::post_request,
};

use leptos::prelude::*;

pub fn request_events_resource(
    request_details: RwSignal<GetEvent>,
) -> LocalResource<Vec<EventInstance>> {
    LocalResource::new(move || request_events(request_details.get()))
}

async fn request_events(request_details: GetEvent) -> Vec<EventInstance> {
    // See if we have any saved events for the requested plant

    return request_events_http(request_details).await;
}

async fn request_events_http(request_details: GetEvent) -> Vec<EventInstance> {
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

    response
}
