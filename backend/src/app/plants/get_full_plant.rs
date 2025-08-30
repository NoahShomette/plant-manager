use axum::{
    extract::{Path, State},
    response::Response,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    plant::{Plant, PlantState},
    InfallibleHistoryItem,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

/// Struct which represents an entire plant
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantDatabase {
    pub name: Json<InfallibleHistoryItem<String>>,
    pub date_created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub id: Uuid,
    pub state: Json<InfallibleHistoryItem<PlantState>>,
}

pub async fn request_plant(Path(plant_id): Path<String>, State(pool): State<PgPool>) -> Response {
    let result: PlantDatabase = match sqlx::query_as(&format!(
        "SELECT id, name, state, date_created, last_modified FROM plants where id ='{}'",
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
        name: result.name.0,
        plant_state: result.state.0,
        date_created: result.date_created,
        last_modified: result.last_modified,
    };
    Response::new(serde_json::ser::to_string(&plant).unwrap().into())
}
