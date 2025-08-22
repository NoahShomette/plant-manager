use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::HistoryItem;

pub mod plant_http;

/// A Plant that has been saved into the database
#[derive(Hash, Serialize, Deserialize)]
pub struct Plant {
    pub name: HistoryItem<String>,
    //pub extra_data: Vec<(String, HistoryItem<ExtraData>)>,
    //pub location: HistoryItem<Location>,
    //pub notes: Vec<String>,
    pub plant_state: HistoryItem<PlantState>,
}

impl Plant {
    pub fn new(name: String, location: Location) -> Plant {
        Plant {
            name: HistoryItem::new(name),
            //extra_data: vec![],
            //location: HistoryItem::new(location),
            //notes: vec![],
            plant_state: HistoryItem::new(PlantState::Alive),
        }
    }

    /// Returns the hashed value of this plant
    pub fn hashed(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum PlantState {
    Alive,
    Retired,
    Gifted,
}

/// Extra data that every plant can have, these can be user defined
#[derive(Hash)]
pub struct ExtraData {
    pub name: String,
    pub data: ExtraDataType,
}

/// The different types of extra data that can be used
#[derive(Hash)]
pub enum ExtraDataType {
    Text(String),
    Date(NaiveTime),
    Number(u32),
}

#[derive(Hash)]
pub struct Location {
    pub name: String,
}

#[derive(Hash)]
pub struct Photo {}
