use chrono::{Local, NaiveDateTime, NaiveTime, Utc};
use leptos::prelude::*;
use reactive_stores::Store;
use shared::events::{
    events_http::{GetEvent, GetEventType, NewEvent},
    EventData, EventType,
};
use thaw::{Button, DatePicker, Input, TimePicker};
use uuid::Uuid;

use crate::{
    data_storage::events::{
        event_storage::{request_events_resource, submit_event, EventStorageContext},
        EventListContext,
    },
    FrontEndState,
};

#[component]
pub fn EventTypeComponent(event_id: Uuid, plant_id: Uuid) -> impl IntoView {
    let (add_mode, set_add_mode) = signal(false);
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();

    let plant_storage_context: EventListContext = expect_context::<EventListContext>();
    let events = plant_storage_context.get_event_list.get();
    let event_type = events
        .0
        .iter()
        .find(|item| item.id == event_id)
        .expect("Plant not found in storage")
        .clone();

    let event_name = event_type.name.clone();

    let event_action = request_events_resource(GetEvent {
        event_type: event_type.id,
        plant_id: plant_id,
        request_details: GetEventType::LastNth(1),
    });

    let resource = move || event_action.get();

    let value = RwSignal::new("".to_string());
    let event_time = RwSignal::new(Local::now().naive_utc());

    // 3 initial states, we are still requesting data, we have received it and we have the data, we received it and we have no events
    view! {
        {move || match resource() {
            Some(data) => {
                match !data.is_empty() {
                    true => {
                        let event_type = event_type.clone();

                        // We have succesfully requested the data
                        // We have events of this type
                        view! {
                            <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
                                <div class="flex flex-row">
                                    <h2 class="text-(--foreground) p-1 text-md font-bold tracking-wide">
                                        {event_name.clone()}
                                    </h2>
                                    <Button on_click=move |_| {
                                        *set_add_mode.write() = !add_mode.get();
                                    }>"Add Event"</Button>
                                </div>

                                <div>
                                    <Show
                                        when=move || add_mode.get()
                                        fallback=move || match &data[0].data {
                                            shared::events::EventData::DateTime => {
                                                view! {
                                                    "Last Watered: "{data[0].event_date.format("%d/%m/%Y %H:%M").to_string()}
                                                }
                                            }
                                            shared::events::EventData::Period(period) => todo!(),
                                            shared::events::EventData::CustomEnum(custom_enum) => {
                                                todo!()
                                            }
                                            shared::events::EventData::Number(_) => todo!(),
                                            shared::events::EventData::String(_) => todo!(),
                                        }
                                    >
                                        {event_data_input(
                                            event_time,
                                            event_type.clone(),
                                            plant_id,
                                            reqwest_client,
                                        )}
                                    </Show>

                                </div>

                            </div>
                        }
                            .into_any()
                    }
                    false => {
                        view! {
                            <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
                                <div class="flex flex-row">
                                    <h2 class="text-(--foreground) p-1 text-md font-bold tracking-wide">
                                        {event_name.clone()}
                                        {event_time
                                            .get_untracked()
                                            .format("%d/%m/%Y %H:%M")
                                            .to_string()}
                                    </h2>
                                </div>
                                {event_data_input(
                                    event_time,
                                    event_type.clone(),
                                    plant_id,
                                    reqwest_client,
                                )}

                            </div>
                        }
                            .into_any()
                    }
                }
            }
            None => {
                // We havent added any events of this type

                // We are still requesting the data
                view! {
                    <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
                        <div class="flex flex-row">
                            <h2 class="text-(--foreground) p-1 text-md font-bold tracking-wide">
                                {event_name.clone()}
                            </h2>
                            <Button on_click=move |_| {
                                *set_add_mode.write() = !add_mode.get();
                            }>"Add Event"</Button>
                        </div>

                        <div>
                            <Show when=move || add_mode.get() fallback=|| view! { view_modess }>
                                <Input value placeholder="Update Name" />
                                <Button on_click=move |_| {
                                    let plant_storage_context: EventStorageContext = expect_context::<
                                        EventStorageContext,
                                    >();
                                }>"Update"</Button>
                            </Show>

                        </div>

                    </div>
                }
                    .into_any()
            }
        }}
    }
}

fn event_data_input(
    event_time: RwSignal<NaiveDateTime>,
    event_type: EventType,
    plant_id: Uuid,
    reqwest_client: Store<FrontEndState>,
) -> impl IntoView {
    view! {
        {
            let local_date = RwSignal::new(Local::now().date_naive());
            let local_time = RwSignal::new(Local::now().time());
            *event_time.write() = local_date.get().and_time(local_time.get());
            let mut event_data = EventData::DateTime;

            view! {
                <div class="flex flex-row">
                    {match event_type.kind {
                        shared::events::EventDataKind::DateTime => {
                            event_data = EventData::DateTime;
                            view! {
                                <DatePicker value=local_date />
                                <TimePicker value=local_time />
                            }
                        }
                        shared::events::EventDataKind::Period => todo!(),
                        shared::events::EventDataKind::CustomEnum => todo!(),
                        shared::events::EventDataKind::Number => todo!(),
                        shared::events::EventDataKind::String => todo!(),
                    }}
                    <Button on_click=move |_| {
                        submit_event(
                            NewEvent {
                                event_type: event_type.id.clone(),
                                plant_id: plant_id.clone(),
                                event_data: event_data.clone(),
                                event_date: event_time.get_untracked(),
                            },
                            reqwest_client.get(),
                        );
                    }>"Add"</Button>

                </div>
            }
        }
    }
}
