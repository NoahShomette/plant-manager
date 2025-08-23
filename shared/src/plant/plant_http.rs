use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
