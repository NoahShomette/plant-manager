use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::{EventData, EventDataKind, EventInstance};

/// HTTP request sent to server to create a new event type
#[derive(Debug, Serialize, Deserialize)]
pub struct NewEventType {
    pub event_type: Uuid,
    pub event_data: EventDataKind,
    pub is_unique: bool,
}

/// HTTP request sent to server to log a new event of the given type for the given plant. The Event Data must match the kind specified by the event type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEvent {
    pub event_type: Uuid,
    pub plant_id: Uuid,
    pub event_data: EventData,
    pub event_date: NaiveDateTime,
}

/// HTTP request sent to server to request events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetEvent {
    pub event_type: Uuid,
    pub plant_id: Uuid,
    pub request_details: GetEventType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GetEventType {
    /// Returns all events up to the given date
    Span(NaiveDateTime, NaiveDateTime),
    /// Returns the last N dates
    LastNth(i32),
    /// Returns all events
    All,
}

/// HTTP response sent from the server in response to the given request
#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventResponse {
    pub events: Vec<EventInstance>,
    pub plant_id: Uuid,
    pub request_details: GetEventType,
}

pub enum GetEventError {
    InfallibleEventHadNoEvents,
    UniqueEventHadNoEvents,
}
