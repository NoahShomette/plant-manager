use leptos::prelude::*;
use shared::events::{
    events_http::{GetEvent, GetEventType},
    PHOTO_EVENT_TYPE_ID, PLANT_NAME_EVENT_ID,
};
use uuid::Uuid;

use crate::{
    components::plant_components::photo::PhotoDisplayComponent,
    data_storage::events::event_storage::request_events_resource,
};

#[component]
pub fn PlantCard(plant_id: Uuid) -> impl IntoView {
    let get_events = RwSignal::new(GetEvent {
        event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });
    let request_events = request_events_resource(get_events);

    let get_events = RwSignal::new(GetEvent {
        event_type: Uuid::parse_str(PHOTO_EVENT_TYPE_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });
    let request_photos = request_events_resource(get_events);

    view! {
        <a href=format!("/plant/{}/view", plant_id.to_string())>
            <div class="bg-(--card) hover:bg-(--accent) p-1 rounded-(--radius) hover:scale-105 transition duration-150">
                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || Suspend::new(async move {
                        let data = request_photos.await;
                        match data.iter().next() {
                            Some(photo) => {
                                view! {
                                    <div class="max-w-[400px]">
                                        <PhotoDisplayComponent photo_location=photo
                                            .data
                                            .expect_kind_string()
                                            .unwrap() />
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! {}.into_any(),
                        }
                    })}
                </Suspense>
                <h2 class="text-(--foreground) py-2 text-base md:text-lg font-bold tracking-wide">

                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || Suspend::new(async move {
                            let data = request_events.await;
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
