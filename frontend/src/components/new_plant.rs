use std::io::Cursor;

use base64::Engine;
use chrono::{Local, Utc};
use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use shared::plant::{plant_http::NewPlant, PlantDemographic};
use thaw::{Button, DatePicker, FileList, Input, Label, Upload};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;

use crate::{
    data_storage::plants::{PlantStorage, PlantStorageContext},
    FrontEndState,
};

#[component]
pub fn NewPlant() -> impl IntoView {
    let value = RwSignal::new("".to_string());

    let submit_response = RwSignal::new("Unknown".to_string());
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let submit_response_2 = RwSignal::new("Unknown".to_string());
    let date_value = RwSignal::new(Local::now().date_naive());
    let click = move |_| {
        spawn_local(submit_new_plant(
            submit_response,
            submit_response_2,
            value.get(),
            reqwest_client.get(),
            plant_storage_context.get_plant_storage.get(),
            plant_storage_context.write_plant_storage,
        ))
    };


    //let uploaded_images = RwSignal::new(Image)

    view! {
        <div class="flex flex-col justify-center py-3 px-5">
            <div class="flex flex-row justify-center items-center py-3 gap-2">
                <Input value placeholder="Name Placeholder" />
                <Label>{move || submit_response.get()}</Label>
            </div>
            <DatePicker value=date_value />

            <Button on_click=click>"Create new Plant"</Button>
            <Label>{move || submit_response_2.get()}</Label>

        </div>
    }
}



async fn submit_new_plant(
    submit_response: RwSignal<String>,
    submit_response_2: RwSignal<String>,
    plant_name: String,
    reqwest_client: FrontEndState,
    mut plant_storage: PlantStorage,
    plant_storage_write: WriteSignal<PlantStorage>,
) {
    if plant_name.len() <= 0 {
        *submit_response.write() = "ERROR: Plant name must be greater than 0".to_string();
        return;
    }

    let Some(response) = reqwest_client
        .client
        .post("http://localhost:8080/plants/new")
        .json(&NewPlant {
            name: plant_name,
            timestamp: Utc::now().naive_utc().and_utc().timestamp(),
            starting_events: vec![],
        })
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        *submit_response.write() = "ERROR".to_string();
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        *submit_response.write() = "ERROR".to_string();
        return;
    };

    let Ok(new_plant) = serde_json::de::from_str::<PlantDemographic>(&body_text) else {
        *submit_response_2.write() = "ERROR Deserializing New Plant".to_string();
        return;
    };

    plant_storage
        .plants
        .insert(new_plant.id, (new_plant.clone(), None));

    *plant_storage_write.write() = plant_storage;

    *submit_response_2.write() = format!("{:?}", new_plant);
}
