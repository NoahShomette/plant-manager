//! Stores a local copy of plants for lowered network usage and faster responses

use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::{
    prelude::{Signal, Write, WriteSignal},
    reactive::spawn_local,
};

use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use shared::plant::{
    plant_http::{VerifyClientPlantList, VerifyClientPlantListResponse},
    Plant, PlantDemographic,
};
use uuid::Uuid;

use crate::FrontEndState;

use leptos::prelude::*;

#[component]
pub fn PlantStorageComponent(children: Children) -> impl IntoView {
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let pv_context: PlantVerificationRequestContext =
        expect_context::<PlantVerificationRequestContext>();

    Effect::new(move |_| {
        spawn_local(verify_client_plants(
            reqwest_client.get_untracked(),
            pv_context.get,
            pv_context.write,
            plant_storage_context.get_plant_list,
            plant_storage_context.write_plant_list,
            plant_storage_context.write_plant_storage,
        ))
    });

    set_interval(
        move || {
            Effect::new(move |_| {
                spawn_local(verify_client_plants(
                    reqwest_client.get_untracked(),
                    pv_context.get,
                    pv_context.write,
                    plant_storage_context.get_plant_list,
                    plant_storage_context.write_plant_list,
                    plant_storage_context.write_plant_storage,
                ))
            });
        },
        Duration::from_secs(60),
    );

    //if (last_requested.get_untracked() + Duration::minutes(1)) < Utc::now().naive_utc() {}

    view! { {children()} }
}

#[derive(Clone, PartialEq)]
pub struct PlantStorageContext {
    pub get_plant_storage: Signal<PlantStorage>,
    pub write_plant_storage: WriteSignal<PlantStorage>,
    pub get_plant_list: Signal<PlantList>,
    pub write_plant_list: WriteSignal<PlantList>,
}

/// Local in memory store of the users plants. Filled with user plants that have already been synced
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PlantStorage {
    pub plants: HashMap<Uuid, (PlantDemographic, Option<Plant>)>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PlantList(pub Vec<Uuid>);

#[derive(Clone, PartialEq)]
pub struct PlantVerificationRequestContext {
    pub get: Signal<PlantVerificationRequests>,
    pub write: WriteSignal<PlantVerificationRequests>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PlantVerificationRequests(pub NaiveDateTime);

impl Default for PlantVerificationRequests {
    fn default() -> Self {
        Self(DateTime::UNIX_EPOCH.naive_utc())
    }
}

async fn verify_client_plants(
    reqwest_client: FrontEndState,
    last_requested: Signal<PlantVerificationRequests>,
    write_last_requested: WriteSignal<PlantVerificationRequests>,
    plant_list: Signal<PlantList>,
    plant_list_write: WriteSignal<PlantList>,
    write_plant_storage: WriteSignal<PlantStorage>,
) {
    let Some(response) = reqwest_client
        .client
        .post("http://localhost:8080/plants/verify-client-list")
        .json(&VerifyClientPlantList {
            last_request: last_requested.get_untracked().0,
        })
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

    let Ok(response) = serde_json::de::from_str::<VerifyClientPlantListResponse>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };
    println!(
        "Received verification of client plant list back {:?}",
        response
    );

    for new_plant in response.new_plants.iter() {
        if plant_list.get_untracked().0.contains(new_plant) {
            continue;
        }
        spawn_local(request_plant_demographic(
            *new_plant,
            reqwest_client.clone(),
            write_plant_storage,
        ));
        // add them to the plants list plus request basic demographic data
    }

    plant_list_write
        .write()
        .0
        .append(&mut response.new_plants.clone());

    for deleted_plants in response.deleted_plants.iter() {
        plant_list_write
            .write()
            .0
            .retain(|list| list != deleted_plants);

        write_plant_storage.write().plants.remove(deleted_plants);
        // Delete them from the client but eventually queue them and then surface them to the user.
        // Ultimately deleted plants shouldnt occur very often but this will prevent a catastrophic loss of a plant if theres a mistake
        // We will need to include some type of intentionally deleted list so that we dont mess up this list
    }

    for changed_plants in response.changed_plants.iter() {
        // refresh basic demographic data? idk exactly
        spawn_local(request_plant_demographic(
            *changed_plants,
            reqwest_client.clone(),
            write_plant_storage,
        ));
    }

    // plant_storage.plants.insert(new_plant.id, new_plant.clone());

    // *plant_storage_write.write() = plant_storage;

    // *submit_response_2.write() = format!("{:?}", new_plant);

    // TODO: Queue a resync of the PlantStorage now. If we've deleted plants then we want to remove them asap and if we've spawned new plants then we want to pull their demographics if we dont have that already
    write_last_requested.write().0 = Utc::now().naive_utc();
}

async fn request_plant_demographic(
    plant_id: Uuid,
    reqwest_client: FrontEndState,
    plant_storage_writer: WriteSignal<PlantStorage>,
) {
    let Some(response) = reqwest_client
        .client
        .get(format!(
            "http://localhost:8080/plants/get_demographic/{}",
            plant_id.to_string()
        ))
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
