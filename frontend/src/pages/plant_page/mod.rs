//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use chrono::Utc;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reactive_stores::Store;
use shared::events::{
    events_http::{GetEvent, GetEventType, NewEvent},
    PLANT_NAME_EVENT_ID, PLANT_STATE_ID,
};

use thaw::{
    Button, ButtonAppearance, ButtonSize, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle, Input,
};
use uuid::Uuid;

use crate::{
    components::plant_components::event::{EventEditComponent, EventViewComponent},
    data_storage::events::{
        event_storage::{request_events_resource, EventStorageContext},
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
    let request_events_resource = request_events_resource(GetEvent {
        event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID).expect("Invalid UUID"),
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });

    let value = RwSignal::new("...".to_string());

    let num_events = RwSignal::new(3);

    let (edit_name, set_edit_name) = signal(false);

    view! {
        {move || {
            if let Some(data) = request_events_resource.get() {
                *value.write() = data.iter().next().unwrap().data.expect_kind_string().unwrap();
            }
        }}
        <div>
            <div>
                <div class="flex flex-row items-center">
                    <Button
                        class="aspect-square p-4 flex-none"
                        size=ButtonSize::Medium
                        icon=icondata::FiEdit
                        on_click=move |_| {
                            *set_edit_name.write() = !edit_name.get();
                        }
                    ></Button>
                    <Show
                        when=move || edit_name.get()
                        fallback=move || {
                            view! {
                                <h2 class="text-(--secondary) p-4 text-5xl font-extrabold tracking-wide italic">
                                    {move || {
                                        match request_events_resource.get() {
                                            Some(data) => {
                                                data.iter()
                                                    .next()
                                                    .unwrap()
                                                    .data
                                                    .expect_kind_string()
                                                    .unwrap()
                                            }
                                            None => "Loading...".to_string(),
                                        }
                                    }}
                                </h2>
                            }
                        }
                    >
                        <Input value placeholder="Update Name" />
                        <Button on_click=move |_| {
                            new_event_click
                                .dispatch((
                                    NewEvent {
                                        event_type: Uuid::parse_str(PLANT_NAME_EVENT_ID)
                                            .expect("Invalid UUID"),
                                        plant_id,
                                        event_data: shared::events::EventData::String(value.get()),
                                        event_date: Utc::now().naive_utc(),
                                    },
                                    reqwest_client.get(),
                                    event_storage_context,
                                ));
                        }>"Update"</Button>
                    </Show>

                </div>
                <div>
                    <For
                        each=move || {
                            let mut list = event_list.get_event_list.get().0.clone();
                            list.retain(|item| {
                                item.id != Uuid::parse_str(PLANT_NAME_EVENT_ID).unwrap()
                                    && item.id != Uuid::parse_str(PLANT_STATE_ID).unwrap()
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
