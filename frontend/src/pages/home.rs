use leptos::prelude::*;

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="bg-background">
            <NewPlant />

        </div>
    }
}
