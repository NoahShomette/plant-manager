use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::EventData;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEventType {
    pub name: String,
    pub timestamp: i64,
}

/// HTTP request sent to server to log a new event of the given type for the given plant. The Event Data must match the kind specified by the event type
#[derive(Debug, Serialize, Deserialize)]
pub struct NewEvent {
    pub event_type: Uuid,
    pub plant_id: Uuid,
    pub event_data: EventData,
}
