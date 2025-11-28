use leptos::prelude::*;
use shared::events::{
    events_http::{GetEvent, GetEventType},
    PLANT_NAME_EVENT_ID,
};
use uuid::Uuid;

use crate::data_storage::events::event_storage::{request_events_resource, PlantEvents};

#[component]
pub fn PlantCard(plant_id: Uuid) -> impl IntoView {
    let local_event_storage: RwSignal<PlantEvents> = RwSignal::new(PlantEvents::default());

    let request_events_resource = request_events_resource(
        GetEvent {
            event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID).expect("Invalid UUID"),
            plant_id: plant_id,
            request_details: GetEventType::LastNth(1),
        },
        local_event_storage,
    );

    view! {
        <a href=format!("/plant/{}/view", plant_id.to_string())>
            <div class="bg-(--card) hover:bg-(--accent) p-3 rounded-(--radius) w-[150px] hover:scale-105 transition duration-150">
                <h2 class="text-(--foreground) p-3 text-lg font-bold tracking-wide">
                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || Suspend::new(async move {
                            let data = request_events_resource.await;
                            view! {
                                {data.iter().next().unwrap().data.expect_kind_string().unwrap()}
                            }
                        })}
                    </Suspense>

                </h2>
            </div>
        </a>
    }
}
