//! Stores a local copy of plants for lowered network usage and faster responses

use std::collections::HashMap;

use gloo_net::http::Request;
use leptos::{
    prelude::{Write, WriteSignal},
    reactive::spawn_local,
};

use serde::{Deserialize, Serialize};
use shared::plant::{Plant, PlantDemographic};
use uuid::Uuid;

use crate::{data_storage::plants::list::PlantListComponent, default_http_request};

use leptos::prelude::*;

pub mod list;
pub mod plants;

#[component]
pub fn PlantStorageComponent(children: Children) -> impl IntoView {
    let (state, set_state) = signal(PlantStorage::default());

    provide_context(PlantStorageContext {
        get_plant_storage: state,
        write_plant_storage: set_state,
    });

    //if (last_requested.get_untracked() + Duration::minutes(1)) < Utc::now().naive_utc() {}

    view! { <PlantListComponent>{children()}</PlantListComponent> }
}

#[derive(Clone, PartialEq)]
pub struct PlantStorageContext {
    pub get_plant_storage: ReadSignal<PlantStorage>,
    pub write_plant_storage: WriteSignal<PlantStorage>,
}

impl PlantStorageContext {
    pub fn request_demographic(&self, plant_id: &Uuid) {
        spawn_local(request_plant_demographic(
            *plant_id,
            self.write_plant_storage,
        ));
    }
}

/// Local in memory store of the users plants. Filled with user plants that have already been synced
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PlantStorage {
    pub plants: HashMap<Uuid, (PlantDemographic, Option<Plant>)>,
}

async fn request_plant_demographic(
    plant_id: Uuid,
    plant_storage_writer: WriteSignal<PlantStorage>,
) {
    let request = Request::get(&format!(
        "http://localhost:8080/plants/get-demographic/{}",
        plant_id.to_string()
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

    let Ok(response) = serde_json::de::from_str::<PlantDemographic>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };

    plant_storage_writer
        .write()
        .plants
        .entry(plant_id)
        .and_modify(|(demo, _)| *demo = response.clone())
        .or_insert((response, None));
}
