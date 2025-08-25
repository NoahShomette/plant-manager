use leptos::prelude::*;
use uuid::Uuid;

use crate::plant_storage::PlantStorageContext;

#[component]
pub fn PlantCard(plant_id: Uuid) -> impl IntoView {
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let plant = plant_storage_context.get_plant_storage.get();
    let plant = plant
        .plants
        .get(&plant_id)
        .expect("Plant not found in storage")
        .clone();

    let plant_name = plant.0.name.clone();

    view! {
        <a href=format!("/plant/view/{}", plant_id.to_string())>
            <div class="bg-(--card) p-3 rounded-(--radius) w-[300px] hover:scale-105 transition duration-150">
                <h2 class="text-(--foreground) p-3 text-lg font-bold tracking-wide">
                    {plant_name}
                </h2>
            </div>
        </a>
    }
}
