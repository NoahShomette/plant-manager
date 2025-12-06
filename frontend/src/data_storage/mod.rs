use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use leptos::{leptos_dom::logging::console_log, prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::{use_event_source, UseEventSourceReturn};
use serde::{Deserialize, Serialize};
use shared::DirtyCache;
use uuid::Uuid;

use crate::{
    data_storage::{events::EventStorageComponent, plants::PlantStorageComponent},
    server_helpers::base_server_addr,
};

pub mod events;
pub mod plants;

#[component]
pub fn AppStorageComponent(children: Children) -> impl IntoView {
    //if (last_requested.get_untracked() + Duration::minutes(1)) < Utc::now().naive_utc() {}
    let (pv, pv_set) = signal(DirtyManager::default());

    provide_context(DirtyManagerContext {
        get: pv,
        write: pv_set,
    });

    let UseEventSourceReturn {
        
        data,
        
        
        ..
    } = use_event_source::<DirtyCache, JsonSerdeCodec>(&format!(
        "{}/dirty-cache",
        base_server_addr()
    ));

    Effect::new(move |_| {
        if let Some(dirty_cache) = data.get() {
            console_log(&format!("Dirty Cache received message"));
            match dirty_cache.cache {
                shared::CacheType::Plant(uuid) => {
                    pv_set.update(|dirty_context| {
                        dirty_context.plants.insert(uuid);
                    });
                }
                shared::CacheType::Event(plant_id, event_id, date_of_event) => {
                    pv_set.update(|dirty_context| {
                        dirty_context
                            .events
                            .entry(plant_id)
                            .and_modify(|(plant_events, _)| {
                                plant_events.insert(event_id);
                            })
                            .or_insert_with(|| {
                                let mut hash = HashSet::new();
                                hash.insert(event_id);
                                (hash, date_of_event)
                            });
                    });
                }
                shared::CacheType::EventType(uuid) => {
                    pv_set.write().event_types.insert(uuid);
                }
            }
        }
    });

    view! {
        <PlantStorageComponent>
            <EventStorageComponent>{children()}</EventStorageComponent>
        </PlantStorageComponent>
    }
}

#[derive(Clone, PartialEq)]
pub struct DirtyManagerContext {
    pub get: ReadSignal<DirtyManager>,
    pub write: WriteSignal<DirtyManager>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct DirtyManager {
    /// Plants that are dirty and need rerequesting
    pub plants: HashSet<Uuid>,
    /// A hashmap that keeps track of the dirtied event types of each plant.
    /// The key to the hashmap is the plant id. The list are all the event types that are dirty and need requesting
    pub events: HashMap<Uuid, (HashSet<Uuid>, NaiveDateTime)>,
    pub event_types: HashSet<Uuid>,
}

impl DirtyManager {
    /// Cleans the given plant events for the given plant.
    pub fn clean_event(&mut self, plant_id: Uuid, event_type: Uuid) {
        if let Some((events, _)) = self.events.get_mut(&plant_id) {
            events.remove(&event_type);
            if events.is_empty() {
                self.events.remove(&plant_id);
            }
        }
    }
}
