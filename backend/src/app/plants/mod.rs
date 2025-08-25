use serde::{Deserialize, Serialize};
use shared::{plant::PlantState, HistoryItem};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

/// Module for creating full Plant instances from the database
pub mod plant;
/// Module for creating plant demographics from the database
pub mod plant_demographic;
pub mod plant_new;
pub mod plants_verification;
