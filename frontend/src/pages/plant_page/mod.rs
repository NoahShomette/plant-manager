//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use std::io::Cursor;

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;
use reactive_stores::Store;
use shared::{
    events::{
        events_http::{GetEvent, GetEventType, NewEvent},
        EventInstance, PHOTO_EVENT_TYPE_ID, PLANT_NAME_EVENT_ID, PLANT_STATE_ID,
    },
    photos::NewPhoto,
};

use thaw::{
    Button, ButtonAppearance, ButtonSize, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle, FileList, Input, Upload,
};
use uuid::Uuid;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;

use crate::{
    components::plant_components::{event::{EventEditComponent, EventViewComponent}, photo::PhotoDisplayComponent},
    data_storage::events::{
        event_storage::{request_events_resource, EventStorageContext, PlantEvents},
        new_event_action, EventListContext,
    },
    FrontEndState,
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
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let event_storage_context: EventStorageContext = expect_context::<EventStorageContext>();

    let new_event_click = new_event_action();
    let event_list: EventListContext = expect_context::<EventListContext>();

    let request_names = request_events_resource(GetEvent {
        event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });

    let request_photos = request_events_resource(GetEvent {
        event_type: Uuid::parse_str(PHOTO_EVENT_TYPE_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(3),
    });

    let canonical_name = RwSignal::new("...".to_string());
    let new_name = RwSignal::new(canonical_name.get_untracked());

    let num_events = RwSignal::new(3);

    let name_input_ref = NodeRef::new();
    let uploaded_image = RwSignal::new(None::<Vec<u8>>);

    let custom_request =
        move |file_list: FileList| spawn_local(submit_new_photos(file_list, uploaded_image));
    let new_photo_action = new_photo_action();

    view! {
        {move || {
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
        }}
        <div>
            <div>
                <div class="flex flex-row items-center">
                    {
                        view! {
                            <input
                                node_ref=name_input_ref
                                type="text"
                                class="text-(--secondary) p-4 text-5xl font-extrabold tracking-wide italic"
                                bind:value=new_name
                                on:blur=move |_| {
                                    if new_name.get() == canonical_name.get() {
                                        return;
                                    }
                                    new_event_click
                                        .dispatch((
                                            NewEvent {
                                                event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID)
                                                    .expect("Invalid UUID"),
                                                plant_id,
                                                event_data: shared::events::EventData::String(
                                                    new_name.get(),
                                                ),
                                                event_date: Utc::now().naive_utc(),
                                            },
                                            reqwest_client.get(),
                                            event_storage_context,
                                        ));
                                }
                                on:keyup=move |event| {
                                    if new_name.get() == canonical_name.get() {
                                        return;
                                    }
                                    if event.key() == "Enter" {
                                        new_event_click
                                            .dispatch((
                                                NewEvent {
                                                    event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID)
                                                        .expect("Invalid UUID"),
                                                    plant_id,
                                                    event_data: shared::events::EventData::String(
                                                        new_name.get(),
                                                    ),
                                                    event_date: Utc::now().naive_utc(),
                                                },
                                                reqwest_client.get(),
                                                event_storage_context,
                                            ));
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
                                <PhotoDisplayComponent
                                    photo_location={event_type.get().expect_kind_string().unwrap()}
                                />
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
                            .dispatch((
                                NewPhoto {
                                    plant_id,
                                    timestamp: Local::now().naive_local().timestamp(),
                                    photo_binary: uploaded_image,
                                },
                                reqwest_client.get(),
                                event_storage_context,
                            ));
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
                                    event_id=event_type.id
                                    plant_id=plant_id
                                    num_events=num_events
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
fn EventDisplayComponent(
    event_id: Uuid,
    plant_id: Uuid,
    num_events: RwSignal<i32>,
) -> impl IntoView {
    let event_storage_context: EventListContext = expect_context::<EventListContext>();
    let events = event_storage_context.get_event_list.get_untracked();
    let event_type = events
        .0
        .iter()
        .find(|item| item.id == event_id)
        .expect("Plant not found in storage")
        .clone();

    let num_events = move || num_events.get();
    let event_action = request_events_resource(GetEvent {
        event_type: event_type.id,
        plant_id: plant_id,
        request_details: GetEventType::LastNth(num_events()),
    });

    let resource = move || event_action.get();

    let events = event_storage_context.get_event_list.get();
    let event_type = events
        .0
        .iter()
        .find(|item| item.id == event_id)
        .expect("Plant not found in storage")
        .clone();

    let event_name = event_type.name.clone();
    let open = RwSignal::new(false);
    view! {
        <div>
            <div class="flex flex-row">
                <h3 class="text-(--secondary) p-4 text-lg font-bold">{event_name.clone()}</h3>
                <Button on_click=move |_| open.set(true)>"New"</Button>
                <Dialog open>
                    <DialogSurface>
                        <DialogBody>
                            <DialogTitle>
                                {
                                    view! {
                                        <div class="flex justify-between">
                                            <h2>{event_name}</h2>
                                            <Button on_click=move |_| open.set(false)>"Close"</Button>
                                        </div>
                                    }
                                }
                            </DialogTitle>
                            <DialogContent>
                                {
                                    view! {
                                        <EventEditComponent event_id=event_id plant_id=plant_id />
                                    }
                                }
                            </DialogContent>
                        </DialogBody>
                    </DialogSurface>
                </Dialog>
            </div>
            {move || match resource() {
                Some(data) => {
                    match !data.is_empty() {
                        true => {
                            let event_type = event_type.clone();

                            // We have succesfully requested the data
                            // We have events of this type
                            view! {
                                <div class="flex flex-col">
                                    <For
                                        each=move || data.clone()
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
                        false => {

                            view! { <p>"No events found..."</p> }
                                .into_any()
                        }
                    }
                }
                None => {
                    // We havent added any events of this type

                    // We are still requesting the data
                    view! {
                        <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
                            <p>"Loading..."</p>

                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

pub fn new_photo_action() -> Action<(NewPhoto, FrontEndState, EventStorageContext), ()> {
    Action::new_local(|input: &(NewPhoto, FrontEndState, EventStorageContext)| {
        new_photo(input.2.clone(), input.1.clone(), input.0.clone())
    })
}

async fn new_photo(
    event_storage: EventStorageContext,
    reqwest_client: FrontEndState,
    new_event: NewPhoto,
) {
    let Some(response) = reqwest_client
        .client
        .post(format!("http://localhost:8080/photos/new"))
        .json(&new_event)
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

    let Ok(response) = serde_json::de::from_str::<EventInstance>(&body_text) else {
        //TODO: Background Error message logging
        return;
    };

    let mut write = event_storage.write_event_storage.write();

    write
        .plants_index
        .entry(new_event.plant_id)
        .and_modify(|entry| {
            entry.add_new_events(vec![response.clone()]);
        })
        .or_insert(PlantEvents::new_from_events(vec![response.clone()]));
}
