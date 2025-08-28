use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlantEventKind {
    pub name: String,
    pub kind: PlantEventDataKind,
}

impl PlantEventKind {
    pub fn new(name: String, kind: PlantEventDataKind) -> PlantEventKind {
        PlantEventKind { name, kind }
    }
    pub fn get(&self) -> &PlantEventDataKind {
        &self.kind
    }
}

/// An event is a saved piece of information concerning some type of repeatable event that has occured. EG watering, repotting, winter dormancy, etc.
///
/// It is part of the record of a plants care and life.
#[derive(Serialize, Deserialize, Clone)]
pub struct PlantEvent {
    pub name: String,
    pub data: PlantEventData,
}

impl PlantEvent {
    pub fn new(name: String, kind: PlantEventData) -> PlantEvent {
        PlantEvent { name, data: kind }
    }
    pub fn get(&self) -> &PlantEventData {
        &self.data
    }
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone)]
pub enum PlantEventDataKind {
    /// A specific day
    Day,
    /// A time period between two dates
    Period,
    CustomEnum,
}

/// The different types of data that a Plant Event can contain that can exist
#[derive(Serialize, Deserialize, Clone)]
pub enum PlantEventData {
    /// A specific day
    Day(NaiveDateTime),
    /// A time period between two dates
    Period(NaiveDateTime, NaiveDateTime),
    CustomEnum(CustomEnum),
}

impl PlantEventData {
    /// Create a new [`PlantEventKind`] of the Day type
    pub fn new_day(day: NaiveDateTime) -> PlantEventData {
        Self::Day(day)
    }

    /// A fallibale constructor that verifies that start is less than end
    pub fn new_period(start: NaiveDateTime, end: NaiveDateTime) -> Option<PlantEventData> {
        if start >= end {
            return None;
        }
        Some(Self::Period(start, end))
    }
}

/// A custom enum, enables user defined multi-choice events
#[derive(Serialize, Deserialize, Clone)]
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
