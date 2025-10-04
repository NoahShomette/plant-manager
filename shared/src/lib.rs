//! Core app features and settings

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod events;
pub mod plant;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DirtyCache {
    pub cache: CacheType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum CacheType {
    /// Plant Id
    Plant(Uuid),
    /// Left: Plant Id - Right: Event Type ID, time of event thats dirty
    Event(Uuid, Uuid, NaiveDateTime),
    EventType(Uuid),
}
