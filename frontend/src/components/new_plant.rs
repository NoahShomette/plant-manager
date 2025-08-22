use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use shared::plant::plant_http::NewPlant;
use thaw::{Button, Input, Label};

use crate::FrontEndState;

#[component]
pub fn NewPlant() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let submit_response = RwSignal::new("Unknown".to_string());
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();

    let click = move |_| {
        spawn_local(submit_new_plant(
            submit_response,
            value.get(),
            reqwest_client.get(),
        ))
    };

    let submit_response_2 = RwSignal::new("Unknown".to_string());

    let other_click = move |_| {
        spawn_local(request_plant(
            submit_response.get(),
            submit_response_2,
            value.get(),
            reqwest_client.get(),
        ))
    };
    view! {
        <div class="flex flex-col justify-center py-3 px-5">
            <div class="flex flex-row justify-center items-center py-3 gap-2">
                <Input value placeholder="Name plant" />
                <Label>{move || submit_response.get()}</Label>
            </div>

            <Button on_click=click>"Create new Plant"</Button>
            <Button on_click=other_click>"Request plant"</Button>
            <Label>{move || submit_response_2.get()}</Label>

        </div>
    }
}

async fn submit_new_plant(
    submit_response: RwSignal<String>,
    plant_name: String,
    reqwest_client: FrontEndState,
) {
    if plant_name.len() <= 0 {
        *submit_response.write() = "ERROR: Plant name must be greater than 0".to_string();
        return;
    }

    let Some(response) = reqwest_client
        .client
        .post("http://localhost:8080/plants/new")
        .json(&NewPlant { name: plant_name })
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

    *submit_response.write() = body_text;
}

async fn request_plant(
    submit_response: String,
    submit_response_2: RwSignal<String>,
    plant_name: String,
    reqwest_client: FrontEndState,
) {
    if plant_name.len() <= 0 {
        *submit_response_2.write() = "ERROR: Plant name must be greater than 0".to_string();
        return;
    }

    let Some(response) = reqwest_client
        .client
        .get(format!(
            "http://localhost:8080/plants/get/{}",
            submit_response
        ))
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        *submit_response_2.write() = "ERROR".to_string();
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        *submit_response_2.write() = "ERROR".to_string();
        return;
    };

    *submit_response_2.write() = body_text;
}
