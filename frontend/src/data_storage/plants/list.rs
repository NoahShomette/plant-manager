//! Stores a local copy of plants for lowered network usage and faster responses

use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::{
    prelude::{Signal, Write, WriteSignal},
    reactive::spawn_local,
    server::codee::string::JsonSerdeCodec,
};

use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use shared::plant::{
    plant_http::{VerifyClientPlantList, VerifyClientPlantListResponse},
    Plant, PlantDemographic,
};
use uuid::Uuid;

use crate::{
    data_storage::plants::{request_plant_demographic, PlantStorage, PlantStorageContext},
    FrontEndState,
};

use leptos::prelude::*;

#[component]
pub fn PlantListComponent(children: Children) -> impl IntoView {
    let (pv, pv_set) = signal(LastDemographicRequest::default());

    provide_context(LastDemographicRequestContext {
        get: pv,
        write: pv_set,
    });
    let (pl_state, pl_set_state) = signal(PlantList::default());

    provide_context(PlantListContext {
        get_plant_list: pl_state,
        write_plant_list: pl_set_state,
    });
    //if (last_requested.get_untracked() + Duration::minutes(1)) < Utc::now().naive_utc() {}
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let plant_list_context: PlantListContext = expect_context::<PlantListContext>();
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let pv_context: LastDemographicRequestContext =
        expect_context::<LastDemographicRequestContext>();
    Effect::new(move |_| {
        spawn_local(get_plant_list(
            reqwest_client.get_untracked(),
            pv_context.get.get_untracked(),
            pv_context.write,
            plant_list_context.get_plant_list,
            plant_list_context.write_plant_list,
            plant_storage_context.write_plant_storage,
        ))
    });

    Effect::new(move |_| {
        spawn_local(get_plant_list(
            reqwest_client.get_untracked(),
            pv_context.get.get_untracked(),
            pv_context.write,
            plant_list_context.get_plant_list,
            plant_list_context.write_plant_list,
            plant_storage_context.write_plant_storage,
        ))
    });

    set_interval(
        move || {
            Effect::new(move |_| {
                spawn_local(get_plant_list(
                    reqwest_client.get_untracked(),
                    pv_context.get.get_untracked(),
                    pv_context.write,
                    plant_list_context.get_plant_list,
                    plant_list_context.write_plant_list,
                    plant_storage_context.write_plant_storage,
                ))
            });
        },
        Duration::from_secs(60),
    );

    view! { {children()} }
}

#[derive(Clone, PartialEq)]
pub struct PlantListContext {
    pub get_plant_list: ReadSignal<PlantList>,
    pub write_plant_list: WriteSignal<PlantList>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct PlantList(pub Vec<Uuid>);

#[derive(Clone, PartialEq)]
pub struct LastDemographicRequestContext {
    pub get: ReadSignal<LastDemographicRequest>,
    pub write: WriteSignal<LastDemographicRequest>,
}

/// Local in memory store of the entire list of the users plants
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LastDemographicRequest(pub NaiveDateTime);

impl Default for LastDemographicRequest {
    fn default() -> Self {
        Self(DateTime::UNIX_EPOCH.naive_utc())
    }
}

async fn get_plant_list(
    reqwest_client: FrontEndState,
    last_requested: LastDemographicRequest,
    last_requested_write: WriteSignal<LastDemographicRequest>,
    plant_list: ReadSignal<PlantList>,
    plant_list_write: WriteSignal<PlantList>,
    write_plant_storage: WriteSignal<PlantStorage>,
) {
    let Some(response) = reqwest_client
        .client
        .get(format!(
            "http://localhost:8080/plants/get-plant-list/{}",
            last_requested.0.and_utc().timestamp()
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

    let Ok(response) = serde_json::de::from_str::<VerifyClientPlantListResponse>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };

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

    for changed_plants in response.events_modified.iter() {
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
    plant_list_write
        .write()
        .0
        .append(&mut response.new_plants.clone());
    last_requested_write.write().0 = Utc::now().naive_utc();
}
