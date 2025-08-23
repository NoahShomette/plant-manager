use leptos::prelude::*;
use thaw::Theme;

use crate::components::gallery::GalleryComponent;
/// Default Home Page
#[component]
pub fn Gallery() -> impl IntoView {
    view! { <GalleryComponent /> }
}
