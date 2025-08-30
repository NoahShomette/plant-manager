use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::plant::PlantState;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlant {
    pub name: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestPlant {
    ByName(String),
    ByUuid(Uuid),
}

/// A request sent by the client for the server to verify the status of its plant list. Should include all the plants the server has
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyClientPlantList {
    pub last_request: NaiveDateTime,
}

/// Response sent by the server to clients for [`VerifyClientPlantList`] requests.
/// Any plants that arent included in this response are assumed to not have changed or already be included on the client
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyClientPlantListResponse {
    /// Plants that the server has that are not included in the clients list
    pub new_plants: Vec<Uuid>,
    /// Plants that the client has but the server doesn't and therefore should be deleted (TODO: prior to deleting pop up a modal asking the client if they want to delete them)
    pub deleted_plants: Vec<Uuid>,
    /// Plants that the server has that have changed since the last time the client requested plant verification (Not entirely sure we will use this yet but it could be helpful)
    pub changed_plants: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModifyPlant {
    ChangeName(String),
    ChangeState(PlantState),
}
