use chrono::NaiveDateTime;
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
#[derive(Serialize, Deserialize, Clone)]
pub struct EventInstance {
    pub name: String,
    pub data: EventData,
}

impl EventInstance {
    pub fn new(name: String, kind: EventData) -> EventInstance {
        EventInstance { name, data: kind }
    }
    pub fn get(&self) -> &EventData {
        &self.data
    }
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum EventDataKind {
    /// A specific day
    Day,
    /// A time period between two dates
    Period,
    CustomEnum,
    Number,
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EventData {
    /// A specific day
    Day(NaiveDateTime),
    /// A time period between two dates
    Period(NaiveDateTime, NaiveDateTime),
    CustomEnum(CustomEnum),
    Number(u64),
}

impl EventData {
    /// Create a new [`PlantEventKind`] of the Day type
    pub fn new_day(day: NaiveDateTime) -> EventData {
        Self::Day(day)
    }

    /// A fallibale constructor that verifies that start is less than end
    pub fn new_period(start: NaiveDateTime, end: NaiveDateTime) -> Option<EventData> {
        if start >= end {
            return None;
        }
        Some(Self::Period(start, end))
    }

    pub fn equals_kind(&self, event_data_kind: EventDataKind) -> bool{
        return match self{
            EventData::Day(_) => event_data_kind == EventDataKind::Day,
            EventData::Period(_, _) => event_data_kind == EventDataKind::Period,
            EventData::CustomEnum(_) => event_data_kind == EventDataKind::CustomEnum,
            EventData::Number(_) => event_data_kind == EventDataKind::Number,
        }
    }
}

/// A custom enum, enables user defined multi-choice events
#[derive(Serialize, Deserialize, Clone, Debug)]
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
