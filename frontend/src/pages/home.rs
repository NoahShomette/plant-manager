use leptos::prelude::*;

use crate::components::{gallery::GalleryComponent, new_plant::NewPlant};
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="bg-background flex flex-col justify-center">
            <NewPlant />
            <GalleryComponent />

        </div>
    }
}
