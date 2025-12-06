use std::time::Duration;

use chrono::{Local, NaiveDateTime};
use leptos::prelude::*;
use shared::events::{events_http::NewEvent, EventData, EventInstance, EventType};
use thaw::{Button, DatePicker, Select, TimePicker};
use uuid::Uuid;

use crate::data_storage::events::{
    new_event_action, EventListContext,
};

#[component]
pub fn EventEditComponent(event_id: Uuid, plant_id: Uuid) -> impl IntoView {
    let event_list_context: EventListContext = expect_context::<EventListContext>();
    let event_type_signal = RwSignal::new(None);

    let events = move || {
        let events = event_list_context.get_event_list.get();
        let event_type = events
            .0
            .iter()
            .find(|item| item.id == event_id)
            .expect("Plant not found in storage")
            .clone();
        event_type_signal.set(Some(event_type.clone()));
    };

    let event_time = RwSignal::new(Local::now().naive_utc());

    // 3 initial states, we are still requesting data, we have received it and we have the data, we received it and we have no events
    view! {
        {events}
        <div class="bg-(--card) p-2 rounded-(--radius) flex flex-col">
            {move || event_data_input(event_time, event_type_signal.get(), plant_id)}
        </div>
    }
}

#[component]
pub fn EventViewComponent(event: EventInstance) -> impl IntoView {
    let (humanized_time, set_humanized_time) = signal(local_time_ago_humanized(event.event_date));

    set_interval(
        move || {
            Effect::new(move |_| {
                set_humanized_time.maybe_update(|og| {
                    let should_update = og != &mut local_time_ago_humanized(event.event_date);
                    *og = local_time_ago_humanized(event.event_date);
                    should_update
                });
            });
        },
        Duration::from_secs(1),
    );

    match &event.data {
        shared::events::EventData::DateTime => view! {
            <div class="flex flex-row items-center p-2">
                <div class="flex flex-col justify-center p-1 mx-2">
                    <div class="text-center">{event.event_date.format("%A").to_string()}</div>
                    <div class="text-center">{event.event_date.format("%B %d").to_string()}</div>
                </div>
                <div class="text-center">{move || humanized_time.get()}</div>

            </div>
        }
        .into_any(),
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

pub fn local_time_ago_humanized(date: NaiveDateTime) -> String {
    let now = Local::now().naive_local();
    let duration = now.signed_duration_since(date.and_utc().naive_local());

    match duration.num_days() {
        0 => match duration.num_hours() {
            0 => match duration.num_minutes() {
                0 => format!("{} seconds ago", duration.num_seconds()),
                _ => format!("{} minutes ago", duration.num_minutes()),
            },
            _ => format!("{} hours ago", duration.num_hours()),
        },
        1 => format!("yesterday"),
        2..=6 => format!("{} days ago", duration.num_days()),
        7..=13 => "1 week ago".to_string(),
        14..=20 => "2 weeks ago".to_string(),
        21..=27 => "3 weeks ago".to_string(),
        28..=30 => "1 month ago".to_string(),
        31..=59 => format!("{} months ago", duration.num_days() / 30),
        60..=364 => format!("{} months ago", duration.num_days() / 30),
        365..=729 => "1 year ago".to_string(),
        _ => format!("{} years ago", duration.num_days() / 365),
    }
}

fn event_data_input(
    event_time: RwSignal<NaiveDateTime>,
    event_type: Option<EventType>,
    plant_id: Uuid,
) -> impl IntoView {
    let new_event_action = new_event_action();

    match event_type {
        Some(event_type) => view! {
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
                                .clone()
                                .dispatch(
                                    NewEvent {
                                        event_type: event_type.id,
                                        plant_id,
                                        event_data,
                                        event_date: event_time.get_untracked(),
                                    },
                                );
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
        .into_any(),
        None => view! {}.into_any(),
    }
}
