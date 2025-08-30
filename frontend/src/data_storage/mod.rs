use leptos::prelude::*;

use crate::data_storage::{events::EventStorageComponent, plants::PlantStorageComponent};

pub mod events;
pub mod plants;

#[component]
pub fn AppStorageComponent(children: Children) -> impl IntoView {
    //if (last_requested.get_untracked() + Duration::minutes(1)) < Utc::now().naive_utc() {}

    view! {
        <PlantStorageComponent>
            <EventStorageComponent>{children()}</EventStorageComponent>
        </PlantStorageComponent>
    }
}
