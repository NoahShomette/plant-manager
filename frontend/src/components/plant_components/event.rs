use chrono::{Local, NaiveDateTime, NaiveTime, Utc};
use leptos::prelude::*;
use reactive_stores::Store;
use shared::events::{
    events_http::{GetEvent, GetEventType, NewEvent},
    EventData, EventInstance, EventType,
};
use thaw::{Button, DatePicker, Input, Select, TimePicker};
use uuid::Uuid;

use crate::{
    data_storage::events::{
        event_storage::{request_events_resource, EventStorageContext},
        new_event_action, EventListContext,
    },
    FrontEndState,
};

#[component]
pub fn EventEditComponent(event_id: Uuid, plant_id: Uuid) -> impl IntoView {
    let plant_storage_context: EventListContext = expect_context::<EventListContext>();
    let events = plant_storage_context.get_event_list.get();
    let event_type = events
        .0
        .iter()
        .find(|item| item.id == event_id)
        .expect("Plant not found in storage")
        .clone();

    let event_name = event_type.name.clone();

    let event_time = RwSignal::new(Local::now().naive_utc());

    // 3 initial states, we are still requesting data, we have received it and we have the data, we received it and we have no events
    view! {
        <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
            {event_data_input(event_time, event_type.clone(), plant_id)}
        </div>
    }
}

#[component]
pub fn EventViewComponent(event: EventInstance) -> impl IntoView {
    // 3 initial states, we are still requesting data, we have received it and we have the data, we received it and we have no events
    match &event.data {
        shared::events::EventData::DateTime => {
            view! { {event.event_date.format("%A %B %d, %Y - %H:%M").to_string()} }.into_any()
        }
        shared::events::EventData::Period(period) => todo!(),
        shared::events::EventData::CustomEnum(custom_enum) => {
            view! { <p>{format!("{}", *custom_enum.selected().unwrap())}</p> }.into_any()
        }
        shared::events::EventData::Number(number) => {
            view! { <p>{format!("{}", number)}</p> }.into_any()
        }
        shared::events::EventData::String(string) => {
            view! { <p>{format!("{}", string)}</p> }.into_any()
        }
    }
}

fn event_data_input(
    event_time: RwSignal<NaiveDateTime>,
    event_type: EventType,
    plant_id: Uuid,
) -> impl IntoView {
    let new_event_action = new_event_action();
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();
    let event_storage_context = expect_context::<EventStorageContext>();

    view! {
        {
            let local_date = RwSignal::new(Local::now().date_naive());
            let local_time = RwSignal::new(Local::now().time());
            let mut event_data = EventData::DateTime;
            let value = RwSignal::new("".to_string());
            view! {
                <div class="flex flex-col">
                    <div class="flex flex-row">

                        {match event_type.kind {
                            shared::events::EventDataKind::DateTime => {
                                event_data = EventData::DateTime;
                                view! {
                                    <DatePicker value=local_date />
                                    <TimePicker value=local_time />
                                }
                                    .into_any()
                            }
                            shared::events::EventDataKind::Period => todo!(),
                            shared::events::EventDataKind::CustomEnum(data) => {
                                event_data = EventData::CustomEnum(data.clone());
                                let custom_enum = data.clone();

                                view! {
                                    <Select value=value>
                                        <For
                                            each=move || { custom_enum.options().clone() }
                                            key=|item| item.clone()
                                            children=|name| {
                                                view! { <option>{format!("{}", name)}</option> }
                                            }
                                        />
                                    </Select>
                                }
                                    .into_any()
                            }
                            shared::events::EventDataKind::Number => todo!(),
                            shared::events::EventDataKind::String => view! {}.into_any(),
                        }}
                    </div>

                    <Button on_click=move |_| {
                        *event_time.write() = local_date
                            .get_untracked()
                            .and_time(local_time.get_untracked());
                        let mut event_data = event_data.clone();
                        if let EventData::CustomEnum(mut custom_enum) = event_data {
                            custom_enum.select_by_string(value.get());
                            event_data = EventData::CustomEnum(custom_enum.clone());
                        }
                        new_event_action
                            .dispatch((
                                NewEvent {
                                    event_type: event_type.id,
                                    plant_id,
                                    event_data,
                                    event_date: event_time.get_untracked(),
                                },
                                reqwest_client.get(),
                                event_storage_context,
                            ));
                    }>
                        {match event_type.is_unique {
                            true => "Change",
                            false => "Add",
                        }}
                    </Button>

                </div>
            }
        }
    }
}
