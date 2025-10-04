use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::CustomEnum;

pub mod plant_http;

/// A Plant that has been saved into the database
#[derive(Debug, Hash, Serialize, Deserialize, Clone, PartialEq)]
pub struct Plant {
    pub id: Uuid,
    /// The date that the plant was created in UTC
    pub date_created: NaiveDateTime,
    /// The date that the plant was last modified in UTC
    pub event_modified: NaiveDateTime,
}

impl Plant {
    pub fn new(name: String, location: Location) -> Plant {
        Plant {
            id: Uuid::new_v4(),
            //extra_data: vec![],
            //location: HistoryItem::new(location),
            //notes: vec![],
            date_created: Utc::now().naive_utc(),
            event_modified: Utc::now().naive_utc(),
        }
    }

    /// Returns the hashed value of this plant
    pub fn hashed(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// A pared down minimal representation of a plant. Used to populate the client with the minimal amount of information needed to display on cards and search
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlantDemographic {
    pub id: Uuid,
    /// The date that the plant was created in UTC
    pub date_created: NaiveDateTime,
    /// The date that the plant was last modified in UTC
    pub event_modified: NaiveDateTime,
    pub name: Option<String>,
    //pub extra_data: Vec<(String, HistoryItem<ExtraData>)>,
    //pub location: HistoryItem<Location>,
    //pub notes: Vec<String>,
    pub plant_state: CustomEnum,
}

impl PlantDemographic {
    pub fn from_plant(
        plant: Plant,
        name: Option<String>,
        plant_state: CustomEnum,
    ) -> PlantDemographic {
        PlantDemographic {
            id: plant.id,
            name: name,
            plant_state: plant_state,
            date_created: plant.date_created,
            event_modified: plant.event_modified,
        }
    }
}

#[derive(Debug, Hash, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum PlantState {
    #[default]
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
