use leptos::prelude::*;

/// Component to view a specific type of event
#[component]
pub fn PhotoDisplayComponent(photo_location: String) -> impl IntoView {
    let mut photo = photo_location.clone();
    photo = photo.split_once(".").unwrap().1.to_string();
    view! {
        <div>
            <img width="fill" src=format!("http://localhost:8080{}", photo) />
        </div>
    }
}
