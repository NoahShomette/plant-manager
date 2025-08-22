use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

use crate::plant::PlantState;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlant {
    pub name: String,
}

#[cfg(feature = "sqlx")]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantResponse {
    pub name: String,
    pub id: Uuid,
    pub state: Json<PlantState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestPlant {
    ByName(String),
    ByUuid(Uuid),
}
