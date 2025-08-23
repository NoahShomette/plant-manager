//! A page for creating new plants

use leptos::prelude::*;

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn NewPlantPage() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-center">
            <NewPlant />
        </div>
    }
}
