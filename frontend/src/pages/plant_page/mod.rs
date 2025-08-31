//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reactive_stores::Store;
use shared::events::EventType;
use shared::plant::plant_http::ModifyPlant;

use thaw::{Button, Input};
use uuid::Uuid;

use crate::{
    components::plant_components::single_event::EventTypeComponent,
    data_storage::{events::EventListContext, plants::PlantStorageContext},
    FrontEndState,
};
/// Default Home Page
#[component]
pub fn PlantPage() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = move || use_params_map().read();
    let id = params().get("id").unwrap_or_default();
    let plant_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => todo!(),
    };

    // TODO: Update plant page to use the full Plant rather than Plant demographic. The plant page should request the full plant from the server if it doesnt have it already
    //    let plant_name = plant.0.name.state().unwrap().1.clone();
    let value = RwSignal::new("".to_string());

    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let click = Action::new_local(|input: &(RwSignal<String>, FrontEndState, Uuid)| {
        modify_name(input.0.get(), input.1.clone(), input.2)
    });
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let event_list: EventListContext = expect_context::<EventListContext>();

    let mut is_requesting = false;

    view! {
        <div>
            {move || {
                let plant_storage_context: PlantStorageContext = expect_context::<
                    PlantStorageContext,
                >();
                if click.pending().get() {
                    is_requesting = true;
                }
                if is_requesting && !click.pending().get() {
                    plant_storage_context.request_demographic(&plant_id);
                    is_requesting = false;
                }
            }} <div>
                <h2 class="text-(--secondary) p-4 text-5xl font-extrabold tracking-wide italic">
                    {move || {
                        let plant = plant_storage_context.get_plant_storage.get();
                        plant
                            .plants
                            .get(&plant_id)
                            .expect("Plant not found in storage")
                            .clone()
                            .0
                            .name
                    }}
                </h2>
                <div>
                    <Input value placeholder="Update Name" />
                    <Button on_click=move |_| {
                        let plant = plant_storage_context.get_plant_storage.get();
                        click.dispatch((value, reqwest_client.get(), plant_id));
                    }>"Update"</Button>
                </div>
                <div>
                    <For
                        each=move || event_list.get_event_list.get().0.clone()
                        key=|item| item.id
                        children= move |event_type| {
                            view! {
                                <EventTypeComponent
                                    event_id=event_type.id
                                    plant_id=plant_id
                                />
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

async fn modify_name(plant_name: String, reqwest_client: FrontEndState, plant_id: Uuid) {
    if plant_name.len() <= 0 {
        return;
    }

    let Some(response) = reqwest_client
        .client
        .post(format!("http://localhost:8080/plants/modify/{}", plant_id))
        .json(&ModifyPlant::ChangeName(plant_name))
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
}
