use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod events_http;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct EventType {
    pub id: Uuid,
    pub name: String,
    pub kind: EventDataKind,
}

impl EventType {
    pub fn new(name: String, kind: EventDataKind) -> EventType {
        EventType {
            name,
            kind,
            id: Uuid::new_v4(),
        }
    }
    pub fn get(&self) -> &EventDataKind {
        &self.kind
    }
}

/// An event is a saved piece of information concerning some type of repeatable event that has occured. EG watering, repotting, winter dormancy, etc.
///
/// It is part of the record of a plants care and life.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EventInstance {
    pub id: Uuid,
    pub event_type_id: Uuid,
    pub plant_id: Uuid,
    pub data: EventData,
    pub event_date: NaiveDateTime,
}

impl EventInstance {
    pub fn new(kind: EventData, event_type_id: Uuid, plant_id: Uuid) -> EventInstance {
        EventInstance {
            data: kind,
            id: Uuid::new_v4(),
            event_type_id,
            plant_id,
            event_date: Utc::now().naive_utc(),
        }
    }
    pub fn get(&self) -> &EventData {
        &self.data
    }
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum EventDataKind {
    /// A specific day
    DateTime,
    /// A time period between two dates
    Period,
    CustomEnum,
    Number,
    String,
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventData {
    /// A specific day, with no other saved information
    DateTime,
    /// A time period between two dates, can be either period start or end
    Period(Period),
    /// A custom enum that provides the user with multi-select functionality
    CustomEnum(CustomEnum),
    /// A number
    Number(f64),
    /// A string
    String(String),
}

impl EventData {
    /// Create a new [`PlantEventKind`] of the Day type
    pub fn new_day() -> EventData {
        Self::DateTime
    }

    /// A fallibale constructor that verifies that start is less than end
    pub fn new_period(period: Period) -> EventData {
        Self::Period(period)
    }

    pub fn equals_kind(&self, event_data_kind: EventDataKind) -> bool {
        return match self {
            EventData::DateTime => event_data_kind == EventDataKind::DateTime,
            EventData::Period(_) => event_data_kind == EventDataKind::Period,
            EventData::CustomEnum(_) => event_data_kind == EventDataKind::CustomEnum,
            EventData::Number(_) => event_data_kind == EventDataKind::Number,
            EventData::String(_) => event_data_kind == EventDataKind::String,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Period {
    PeriodStart,
    PeriodEnd,
}

/// A custom enum, enables user defined multi-choice events
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomEnum {
    options: Vec<String>,
    selected: usize,
}

impl CustomEnum {
    /// Create a new custom enum based on the given options
    pub fn new(options: Vec<String>) -> Option<CustomEnum> {
        if options.len() == 0 {
            return None;
        }
        Some(Self {
            options,
            selected: 0,
        })
    }

    /// Select an option. Silently fails if the index was invalid
    pub fn select(&mut self, index: usize) {
        if self.options.len() >= index {
            self.selected = index;
        };
    }

    /// Returns the list of options
    pub fn options(&self) -> &Vec<String> {
        &self.options
    }

    /// Returns the selected option if it is a valid index
    pub fn selected(&self) -> Option<&String> {
        self.options.get(self.selected)
    }
}
