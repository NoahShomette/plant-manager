//! Generic gallery component that shows users plants with customizable options for what settings to show alongside them
use leptos::prelude::*;
use thaw::Pagination;

use crate::{components::plant_card::PlantCard, data_storage::plants::PlantStorageContext};

#[component]
pub fn GalleryComponent() -> impl IntoView {
    let plant_storage_context: PlantStorageContext = expect_context::<PlantStorageContext>();
    //let plant = move || plant_storage_context.get_plant_storage.get();
    let page_count =
        move || 1usize.max(plant_storage_context.get_plant_storage.get().plants.len() / 20);
    let current_page = RwSignal::new(1);
    view! {
        <div class="container flex flex-col items-center self-center">
            <div class="grid grid-flow-col grid-cols-2 md:grid-cols-4 gap-3 p-3 justify-center ">
                <For
                    each=move || plant_storage_context.get_plant_storage.get().plants.clone()
                    key=|item| item.0
                    children=|(id, _item)| {
                        view! { <PlantCard plant_id=id /> }
                    }
                />
            </div>
            <Pagination page=current_page page_count=1 sibling_count=1 class="justify-center" />
        </div>
    }
}
