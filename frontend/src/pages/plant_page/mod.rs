//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::plant_storage::PlantStorageContext;
/// Default Home Page
#[component]
pub fn PlantPage() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let plant_id = match Uuid::parse_str(&id()) {
        Ok(id) => id,
        Err(_) => todo!(),
    };
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();

    let plant = plant_storage_context.get_plant_storage.get();
    let plant = plant
        .plants
        .get(&plant_id)
        .expect("Plant not found in storage")
        .clone();

    let plant_name = plant.0.name.clone();

    // TODO: Update plant page to use the full Plant rather than Plant demographic. The plant page should request the full plant from the server if it doesnt have it already
    //    let plant_name = plant.0.name.state().unwrap().1.clone();

    view! { <p>{plant_name}</p> }
}
