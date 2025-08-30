use axum::{
    extract::{Path, State},
    response::Response,
};
use chrono::Utc;
use shared::{
    plant::{plant_http::NewPlant, PlantState},
    InfallibleHistoryItem,
};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use crate::app::plants::get_demographic::request_plant_demographic;

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn new_plant(
    State(pool): State<PgPool>,
    axum::Json(new_plant): axum::Json<NewPlant>,
) -> Response {
    let plant_uuid = Uuid::new_v4();
    let result = sqlx::query("INSERT INTO plants(name, id, state, date_created, last_modified) VALUES ($1, $2, $3, $4, $4)")
        .bind(Json(InfallibleHistoryItem::new_with_timestamp(
            new_plant.name,
            new_plant.timestamp,
        )))
        .bind(plant_uuid.clone())
        .bind(Json(InfallibleHistoryItem::new_with_timestamp(
            PlantState::Alive,
            new_plant.timestamp,
        ))).bind(Utc::now().naive_utc())
        .execute(&pool)
        .await
        .unwrap();

    let new_plant = request_plant_demographic(
        Path::from(axum::extract::Path(plant_uuid.to_string())),
        State(pool),
    )
    .await;

    println!("New Plant Registered: {:?}", result);
    new_plant
}
