//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use std::io::Cursor;

use chrono::{Local, Utc};
use gloo_net::http::Request;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;
use shared::{
    events::{
        events_http::{GetEvent, GetEventType, NewEvent},
        EventInstance, EventType, PHOTO_EVENT_TYPE_ID, PLANT_NAME_EVENT_ID, PLANT_STATE_ID,
    },
    photos::NewPhoto,
};

use thaw::{
    Button, Dialog, DialogBody, DialogContent, DialogSurface, DialogTitle, FileList, Upload,
};
use uuid::Uuid;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;

use crate::{
    components::plant_components::{
        event::{EventEditComponent, EventViewComponent},
        photo::PhotoDisplayComponent,
    },
    data_storage::events::{
        event_storage::{request_events_resource, PlantEvents},
        new_event_action, EventListContext,
    },
    default_http_request,
};
/// Default Home Page
#[component]
pub fn PlantPage() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = move || use_params_map().read_untracked();
    let id = params().get("id").unwrap_or_default();
    let plant_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => todo!(),
    };

    let new_event_click = new_event_action();
    let event_list: EventListContext = expect_context::<EventListContext>();

    let get_events = RwSignal::new(GetEvent {
        event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });
    let request_names = request_events_resource(get_events);

    let get_events = RwSignal::new(GetEvent {
        event_type: Uuid::parse_str(PHOTO_EVENT_TYPE_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(3),
    });
    let request_photos = request_events_resource(get_events);

    let canonical_name = RwSignal::new("...".to_string());
    let new_name = RwSignal::new(canonical_name.get_untracked());

    Effect::new(move || {
        if let Some(data) = request_names.get() {
            *canonical_name.write() = data
                .iter()
                .next()
                .unwrap()
                .data
                .expect_kind_string()
                .unwrap();
            *new_name.write() = canonical_name.get();
        }
    });

    let num_events = RwSignal::new(3);

    let name_input_ref = NodeRef::new();
    let uploaded_image = RwSignal::new(None::<Vec<u8>>);

    let custom_request =
        move |file_list: FileList| spawn_local(submit_new_photos(file_list, uploaded_image));
    let new_photo_action = new_photo_action();

    view! {
        <div>
            <div>
                <div class="flex flex-row items-center">
                    {
                        view! {
                            <input
                                node_ref=name_input_ref
                                type="text"
                                class="text-secondary p-4 text-5xl font-extrabold tracking-wide italic"
                                bind:value=new_name
                                on:blur=move |_| {
                                    if new_name.get() == canonical_name.get() {
                                        return;
                                    }
                                    new_event_click
                                        .clone()
                                        .dispatch(NewEvent {
                                            event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID)
                                                .expect("Invalid UUID"),
                                            plant_id,
                                            event_data: shared::events::EventData::String(
                                                new_name.get(),
                                            ),
                                            event_date: Utc::now().naive_utc(),
                                        });
                                }
                                on:keyup=move |event| {
                                    if new_name.get() == canonical_name.get() {
                                        return;
                                    }
                                    if event.key() == "Enter" {
                                        new_event_click
                                            .clone()
                                            .dispatch(NewEvent {
                                                event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID)
                                                    .expect("Invalid UUID"),
                                                plant_id,
                                                event_data: shared::events::EventData::String(
                                                    new_name.get(),
                                                ),
                                                event_date: Utc::now().naive_utc(),
                                            });
                                        if let Some(input) = name_input_ref.get() {
                                            let _ = input.blur();
                                        }
                                    }
                                }
                            />
                        }
                    }

                </div>

                <div>
                    <For
                        each=move || {
                            if let Some(data) = request_photos.get() { data } else { vec![] }
                        }
                        key=|item| item.id
                        children=move |event_type| {
                            view! {
                                <PhotoDisplayComponent photo_location=event_type
                                    .get()
                                    .expect_kind_string()
                                    .unwrap() />
                            }
                        }
                    />
                    <Upload custom_request>
                        <Button>"Select Photos"</Button>
                    </Upload>
                    <Button on_click=move |_| {
                        let Some(uploaded_image) = uploaded_image.get_untracked() else {
                            return;
                        };
                        new_photo_action
                            .dispatch(NewPhoto {
                                plant_id,
                                timestamp: Local::now().naive_local().and_utc().timestamp(),
                                photo_binary: uploaded_image,
                            });
                    }>"Upload"</Button>
                </div>

                <div>
                    <For
                        each=move || {
                            let mut list = event_list.get_event_list.get().0.clone();
                            list.retain(|item| {
                                item.id != Uuid::parse_str(PLANT_NAME_EVENT_ID).unwrap()
                                    && item.id != Uuid::parse_str(PLANT_STATE_ID).unwrap()
                                    && item.id != Uuid::parse_str(PHOTO_EVENT_TYPE_ID).unwrap()
                            });
                            list
                        }
                        key=|item| item.id
                        children=move |event_type| {
                            view! {
                                <EventDisplayComponent
                                    event_type=event_type.clone()
                                    plant_id=plant_id
                                    num_events=num_events.get()
                                />
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

async fn submit_new_photos(file_list: FileList, uploaded_image: RwSignal<Option<Vec<u8>>>) {
    for file_index in 0..file_list.length() {
        let Some(file) = file_list.get(file_index) else {
            continue;
        };

        let Ok(file_binary) = JsFuture::from(file.array_buffer()).await else {
            continue;
        };
        let array = Uint8Array::new(&file_binary);
        let Ok(image) = image::load_from_memory(&array.to_vec()) else {
            continue;
        };
        let mut buf: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();

        uploaded_image.set(Some(buf));
        //let image = ImageReader::new(buffer.);

        //ImageReader::new(file.stream());
    }
}

/// Component to view a specific type of event
#[component]
fn EventDisplayComponent(event_type: EventType, plant_id: Uuid, num_events: i32) -> impl IntoView {
    let get_events = RwSignal::new(GetEvent {
        event_type: event_type.id,
        plant_id: plant_id,
        request_details: GetEventType::LastNth(num_events),
    });

    let (events, set_events) = signal(vec![]);

    let event_action = request_events_resource(get_events);

    Effect::new(move || {
        if let Some(events) = event_action.get() {
            set_events.set(events);
        }
    });

    let open = RwSignal::new(false);
    view! {
        <div>
            <div class="flex flex-row">
                <h3 class="text-secondary p-4 text-lg font-bold">{event_type.name.clone()}</h3>
                <Button on_click=move |_| open.set(true)>"New"</Button>
                <Dialog open>
                    <DialogSurface>
                        <DialogBody>
                            <DialogTitle>
                                {
                                    view! {
                                        <div class="flex justify-between">
                                            <h2>{event_type.name.clone()}</h2>
                                            <Button on_click=move |_| open.set(false)>"Close"</Button>
                                        </div>
                                    }
                                }
                            </DialogTitle>
                            <DialogContent>
                                {
                                    view! {
                                        <EventEditComponent
                                            event_id=event_type.id
                                            plant_id=plant_id
                                        />
                                    }
                                }
                            </DialogContent>
                        </DialogBody>
                    </DialogSurface>
                </Dialog>
            </div>

            {move || match events.get().is_empty() {
                false => {
                    // We have succesfully requested the data
                    // We have events of this type
                    view! {
                        <div class="flex flex-col">
                            <For
                                each=move || events.get()
                                key=|item| item.id
                                children=move |event| {
                                    view! {
                                        <div>
                                            <EventViewComponent event />
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }
                        .into_any()
                }
                true => view! { <p>"No events found..."</p> }.into_any(),
            }}
        </div>
    }
}

pub fn new_photo_action() -> Action<NewPhoto, ()> {
    Action::new_local(|input: &NewPhoto| new_photo(input.clone()))
}

async fn new_photo(new_event: NewPhoto) {
    let request = Request::post(&format!("http://localhost:8080/photos/new"));
    let request = default_http_request(request);

    let Some(request_with_json) = request
        .json(&new_event)
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        return;
    };

    let Some(response) = request_with_json
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        return;
    };

    let Ok(_response) = serde_json::de::from_str::<EventInstance>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };
}
