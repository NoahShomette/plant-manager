use leptos::prelude::*;
use thaw::{Link, Theme};
use uuid::Uuid;

use crate::plant_storage::PlantStorageContext;

#[component]
pub fn PlantCard(plant_id: Uuid) -> impl IntoView {
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let theme = Theme::use_rw_theme();

    let plant = plant_storage_context.get.get();
    let plant = plant
        .hashmap
        .get(&plant_id)
        .expect("Plant not found in storage")
        .clone();

    let plant_name = plant.name.state().unwrap().1.clone();

    view! {
        <Link href=format!("/plant/view/{}", plant_id.to_string())>
            <div class="bg-(--card) p-3 rounded-(--radius) w-[300px]">
                <h1>{plant_name}</h1>
            </div>
        </Link>
    }
}
