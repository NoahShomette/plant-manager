use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    plant::{Plant, PlantDemographic, PlantState},
    InfallibleHistoryItem,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

/// Struct which represents an entire plant
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantDatabase {
    pub name: Json<InfallibleHistoryItem<String>>,
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub state: Json<InfallibleHistoryItem<PlantState>>,
}

pub async fn request_plant_demographic(
    Path(plant_id): Path<String>,
    State(pool): State<PgPool>,
) -> Response {
    let plant_id = match Uuid::parse_str(&plant_id) {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };
    let result: PlantDatabase = match sqlx::query_as(&format!(
        "SELECT id, name, state, date_created, last_modified FROM plants where id ='{}'",
        plant_id
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(plant) => plant,
        Err(error) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error.to_string()))
                .unwrap();
        }
    };

    println!("New Plant Registered: {:?}", result);
    let plant: PlantDemographic = Plant {
        id: result.id,
        name: result.name.0,
        plant_state: result.state.0,
        date_created: result.date_created,
        last_modified: result.last_modified,
    }
    .into();
    Response::new(serde_json::ser::to_string(&plant).unwrap().into())
}
