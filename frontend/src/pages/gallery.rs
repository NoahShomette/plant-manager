use leptos::prelude::*;

use crate::components::gallery::GalleryComponent;
/// Default Home Page
#[component]
pub fn Gallery() -> impl IntoView {
    view! { <GalleryComponent /> }
}
