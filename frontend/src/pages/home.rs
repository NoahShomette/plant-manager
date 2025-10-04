use leptos::prelude::*;
use shared::events::{CustomEnum, EventData};

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="bg-background">
            {serde_json::to_string(&EventData::CustomEnum(CustomEnum::new(vec!["Alive", "Retired", "Gifted"]).unwrap()))}
            <NewPlant />

        </div>
    }
}
