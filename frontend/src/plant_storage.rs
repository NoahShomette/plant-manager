//! Stores a local copy of plants for lowered network usage and faster responses

use std::collections::HashMap;

use leptos::prelude::{Signal, WriteSignal};
use serde::{Deserialize, Serialize};
use shared::plant::Plant;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct PlantStorageContext {
    pub get: Signal<PlantStorage>,
    pub write: WriteSignal<PlantStorage>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PlantStorage {
    pub hashmap: HashMap<Uuid, Plant>,
}
