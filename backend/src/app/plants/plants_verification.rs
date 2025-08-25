use axum::{
    extract::{Path, State},
    response::Response,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    plant::{
        plant_http::{NewPlant, VerifyClientPlantList, VerifyClientPlantListResponse},
        PlantState,
    },
    HistoryItem,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantId {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub async fn client_verify_plants(
    State(pool): State<PgPool>,
    axum::Json(client_request): axum::Json<VerifyClientPlantList>,
) -> Response {
    let result: Vec<PlantId> =
        sqlx::query_as(r#"SELECT id, date_created, last_modified FROM plants"#)
            .fetch_all(&pool)
            .await
            .unwrap();

    let mut new_plants = vec![];
    let mut deleted_plants = vec![];
    let mut changed_plants = vec![];

    // If the
    for record in result.iter() {
        if client_request.last_request < record.date_created {
            new_plants.push(record.id);
        }
    }

    let response = VerifyClientPlantListResponse {
        new_plants,
        deleted_plants,
        changed_plants,
    };
    Response::new(serde_json::ser::to_string(&response).unwrap().into())
}
