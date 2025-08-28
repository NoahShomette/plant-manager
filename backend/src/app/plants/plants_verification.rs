use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeletedPlantId {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
}

pub async fn client_verify_plants(
    State(pool): State<PgPool>,
    axum::Json(client_request): axum::Json<VerifyClientPlantList>,
) -> Response {
    let new_plants: Vec<PlantId> = match sqlx::query_as(
        r#"SELECT id, date_created, last_modified FROM plants WHERE date_created >= $1"#,
    )
    .bind(client_request.last_request)
    .fetch_all(&pool)
    .await
    {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    let deleted_plants: Vec<DeletedPlantId> = match sqlx::query_as(
        r#"SELECT id, date_deleted FROM deleted_plants WHERE date_deleted >= $1"#,
    )
    .bind(client_request.last_request)
    .fetch_all(&pool)
    .await
    {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    let response = VerifyClientPlantListResponse {
        new_plants: new_plants.iter().map(|item| item.id).collect(),
        deleted_plants: deleted_plants.iter().map(|item| item.id).collect(),
        changed_plants: vec![],
    };
    Response::new(serde_json::ser::to_string(&response).unwrap().into())
}
