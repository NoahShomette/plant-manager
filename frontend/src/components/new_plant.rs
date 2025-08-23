use chrono::{Local, Utc};
use image::ImageReader;
use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use shared::plant::{plant_http::NewPlant, Plant};
use thaw::{Button, DatePicker, FileList, Input, Label, Theme, Upload};

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
    let date_value = RwSignal::new(Local::now().date_naive());
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

    //let uploaded_images = RwSignal::new(Image)

    let custom_request = move |file_list: FileList| {
        let len = file_list.length();
        for file_index in 0..file_list.length(){
            let Some(file) = file_list.get(file_index) else{
                break;
            };

            let buffer = file.stream();
            
            ImageReader::new(file.stream());

        }
    };

    view! {
        <div class="flex flex-col justify-center py-3 px-5">
            <div class="flex flex-row justify-center items-center py-3 gap-2">
                <Input value placeholder="Name Placeholder" />
                <Label>{move || submit_response.get()}</Label>
            </div>
            <DatePicker value=date_value />
            <Upload custom_request>
                <Button>"upload"</Button>
            </Upload>
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
