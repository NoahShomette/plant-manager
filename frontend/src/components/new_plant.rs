use chrono::Utc;
use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use shared::plant::{plant_http::NewPlant, Plant};
use thaw::{Button, Input, Label};

use crate::{
    plant_storage::{PlantStorage, PlantStorageContext},
    FrontEndState,
};

#[component]
pub fn NewPlant() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let submit_response = RwSignal::new("Unknown".to_string());
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let submit_response_2 = RwSignal::new("Unknown".to_string());

    let click = move |_| {
        spawn_local(submit_new_plant(
            submit_response,
            submit_response_2,
            value.get(),
            reqwest_client.get(),
            plant_storage_context.get.get(),
            plant_storage_context.write,
        ))
    };

    view! {
        <div class="flex flex-col justify-center py-3 px-5">
            <div class="flex flex-row justify-center items-center py-3 gap-2">
                <Input value placeholder="Name plant" />
                <Label>{move || submit_response.get()}</Label>
            </div>

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

    let Ok(new_plant) = serde_json::de::from_str::<Plant>(&body_text) else {
        *submit_response_2.write() = "ERROR Deserializing New Plant".to_string();
        return;
    };

    plant_storage
        .hashmap
        .insert(new_plant.id, new_plant.clone());

    *plant_storage_write.write() = plant_storage;

    *submit_response_2.write() = format!("{:?}", new_plant);
}
