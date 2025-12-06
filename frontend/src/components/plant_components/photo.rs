use leptos::prelude::*;

use crate::server_helpers::base_server_addr;

/// Component to view a specific type of event
#[component]
pub fn PhotoDisplayComponent(photo_location: String) -> impl IntoView {
    let mut photo = photo_location.clone();
    photo = photo.split_once(".").unwrap().1.to_string();
    view! {
            <img class="rounded-(--radius) object-cover w-full h-full" src=format!("{}{}", base_server_addr(), photo) />
    }
}
