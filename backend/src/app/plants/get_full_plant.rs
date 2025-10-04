use axum::{
    extract::{Path, State},
    response::Response,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::plant::{Plant, PlantState};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

/// Struct which represents an entire plant
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantDatabase {
    pub date_created: NaiveDateTime,
    pub event_modified: NaiveDateTime,
    pub id: Uuid,
}

pub async fn request_plant(Path(plant_id): Path<String>, State(pool): State<PgPool>) -> Response {
    let result: PlantDatabase = match sqlx::query_as(&format!(
        "SELECT id, date_created, event_modified FROM plants where id ='{}'",
        Uuid::parse_str(&plant_id).unwrap()
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(plant) => plant,
        Err(error) => {
            return Response::new(
                serde_json::ser::to_string(&error.to_string())
                    .unwrap()
                    .into(),
            )
        }
    };

    println!("New Plant Registered: {:?}", result);
    let plant = Plant {
        id: result.id,
        date_created: result.date_created,
        event_modified: result.event_modified,
    };
    Response::new(serde_json::ser::to_string(&plant).unwrap().into())
}
