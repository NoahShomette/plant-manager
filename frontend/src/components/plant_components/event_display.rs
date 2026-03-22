use leptos::prelude::*;
use shared::events::{
    events_http::{GetEvent, GetEventType},
    EventType,
};
use thaw::{Button, Dialog, DialogBody, DialogContent, DialogSurface, DialogTitle};
use uuid::Uuid;

use crate::{
    components::plant_components::event::{EventEditComponent, EventViewComponent},
    data_storage::events::{
        event_storage::request_events_resource, new_event_action, EventListContext,
    },
};

/// Component to view a specific type of event
#[component]
pub fn EventDisplayComponent(event_type: EventType, plant_id: Uuid, num_events: i32) -> impl IntoView {
    let get_events = RwSignal::new(GetEvent {
        event_type: event_type.id,
        plant_id: plant_id,
        request_details: GetEventType::LastNth(num_events, 0),
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
        <div class="w-full flex flex-col">
            <div class="flex flex-row items-center">
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
