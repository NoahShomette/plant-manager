use crate::{
    components::plant_components::photo::PhotoDisplayComponent,
    data_storage::events::event_storage::request_events_resource,
};
use leptos::prelude::*;
use leptos_use::{breakpoints_tailwind, use_breakpoints, BreakpointsTailwind};
use random_color::RandomColor;
use shared::events::{
    events_http::{GetEvent, GetEventType},
    PHOTO_EVENT_TYPE_ID, PLANT_NAME_EVENT_ID,
};
use thaw::Icon;
use uuid::Uuid;
use BreakpointsTailwind::*;

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
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let larger_than_sm = screen_width.gt(Sm);
    let larger_than_md = screen_width.gt(Md);
    let icon_size = signal("75px");

    Effect::new(move || {
        if larger_than_md.get() {
            icon_size.1.set("150px");
        } else if larger_than_sm.get() {
            icon_size.1.set("100px");
        } else {
            icon_size.1.set("75px");
        }
    });
    let mut random_color = RandomColor::new();
    random_color.luminosity(random_color::options::Luminosity::Dark);
    random_color.seed(plant_id.to_string());
    let plant_color = random_color.to_hex();
    let plant_color = signal(plant_color);

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
                                    <div class="m-2 max-w-[400px] aspect-square flex justify-center content-center">
                                        <PhotoDisplayComponent photo_location=photo
                                            .data
                                            .expect_kind_string()
                                            .unwrap() />
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! {
                                <div class={format!("m-2 rounded-(--radius) max-w-[400px] aspect-square flex justify-center content-center")} style={format!("background: {}", plant_color.0.get())}>
                                    <Icon icon=icondata::ChPlantPot width=icon_size.0.get() height=icon_size.0.get() class="text-(--foreground)"></Icon>
                                </div>
                            }.into_any(),
                        }
                    })}
                </Suspense>
                <h2 class="text-(--foreground) p-2 text-base md:text-lg font-bold tracking-wide">
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
