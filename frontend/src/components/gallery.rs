//! Generic gallery component that shows users plants with customizable options for what settings to show alongside them
use leptos::prelude::*;

use crate::{components::plant_card::PlantCard, plant_storage::PlantStorageContext};

#[component]
pub fn GalleryComponent() -> impl IntoView {
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    let plant = plant_storage_context.get.get();
    let plants_iter = plant.hashmap.clone();
    view! {
        <div class="flex flex-wrap gap-3 p-3 justify-center">
            <For
                each=move || plants_iter.clone()
                key=|item| item.0
                children=|(id, _item)| {
                    view! { <PlantCard plant_id=id /> }
                }
            />
        </div>
    }
}
