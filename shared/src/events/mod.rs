use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod events_http;

pub static PLANT_STATE_ID: &str = "1a5c53bb-18c2-4789-8ba4-9bbfc4bc2371";
pub static PLANT_NAME_EVENT_ID: &str = "a501afa2-1959-4f1e-9706-abe97eb85263";
pub static BIRTHDAY_EVENT_ID: &str = "700866fd-a8b8-4cef-af5b-1752a1434129";
pub static REPOTTED_EVENT_ID: &str = "1e7c1c14-dddd-4658-be0a-5c20726b4d16";
pub static WATERED_EVENT_ID: &str = "9c8c6cfc-e111-44c2-9b5c-f5d84ae2da7a";
pub static PHOTO_EVENT_TYPE_ID: &str = "77271e34-e207-47cd-b360-f1db84db4f7e";

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct EventType {
    pub id: Uuid,
    pub name: String,
    pub kind: EventDataKind,
    /// Is this event type deletable by the user
    pub deletable: bool,
    /// Is this even type modifiable by the user
    pub modifiable: bool,
    /// Is this event type unique - there can be only one event type
    pub is_unique: bool,
}

impl EventType {
    pub fn new(
        name: String,
        kind: EventDataKind,
        deletable: bool,
        modifiable: bool,
        unique: bool,
    ) -> EventType {
        EventType {
            name,
            kind,
            id: Uuid::new_v4(),
            deletable,
            modifiable,
            is_unique: unique,
        }
    }

    pub fn table_name(&self) -> &str {
        match self.is_unique {
            true => "events_unique",
            false => "events",
        }
    }

    pub fn get(&self) -> &EventDataKind {
        &self.kind
    }
    pub fn modifiable(&self) -> bool {
        self.modifiable
    }
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }
    pub fn deletable(&self) -> bool {
        self.deletable
    }
}

/// An event is a saved piece of information concerning some type of repeatable event that has occured. EG watering, repotting, winter dormancy, etc.
///
/// It is part of the record of a plants care and life.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EventInstance {
    /// The id of this specific event instance. Used to uniquely identify this event and its data
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
    CustomEnum(CustomEnum),
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
            EventData::Number(_) => event_data_kind == EventDataKind::Number,
            EventData::String(_) => event_data_kind == EventDataKind::String,
            _ => true,
        };
    }

    pub fn expect_kind_string(&self) -> Option<String> {
        return match self {
            EventData::String(string) => Some(string.clone()),
            _ => None,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Period {
    PeriodStart,
    PeriodEnd,
}

/// A custom enum, enables user defined multi-choice events
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CustomEnum {
    options: Vec<String>,
    selected: usize,
}

impl CustomEnum {
    pub fn plant_state() -> CustomEnum {
        CustomEnum {
            options: vec![
                "Alive".to_string(),
                "Retired".to_string(),
                "Gifted".to_string(),
            ],
            selected: 0,
        }
    }

    /// Create a new custom enum based on the given options
    pub fn new(options: Vec<&str>) -> Option<CustomEnum> {
        if options.len() == 0 {
            return None;
        }
        Some(Self {
            options: options.iter().map(|e| e.to_string()).collect(),
            selected: 0,
        })
    }

    /// Select an option. Silently fails if the index was invalid
    pub fn select(&mut self, index: usize) {
        if self.options.len() >= index {
            self.selected = index;
        };
    }

    /// Select an option based on string matching the options
    pub fn select_by_string(&mut self, string: String) {
        for (index, option) in self.options.iter().enumerate() {
            if option == &string {
                self.selected = index;
            }
        }
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
