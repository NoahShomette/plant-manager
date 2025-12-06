use axum::{
    body::Body,
    extract::{RawPathParams, State},
    http::StatusCode,
    response::Response,
};
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use shared::plant::plant_http::VerifyClientPlantListResponse;
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantId {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub event_modified: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeletedPlantId {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
}

pub async fn get_plant_list(params: RawPathParams, State(pool): State<PgPool>) -> Response {
    let timestamp = match params.iter().find(|(name, _data)| *name == "timestamp") {
        Some(result) => match result.1.parse::<i64>() {
            Ok(result) => match DateTime::from_timestamp(result, 0) {
                Some(result) => result,
                None => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Improper address".to_string()))
                        .unwrap();
                }
            },
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(err.to_string()))
                    .unwrap();
            }
        },
        None => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Improper address".to_string()))
                .unwrap();
        }
    };

    let new_plants: Vec<PlantId> = match sqlx::query_as(
        r#"SELECT id, date_created, event_modified FROM plants WHERE date_created >= $1"#,
    )
    .bind(timestamp.naive_utc())
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
    .bind(timestamp.naive_utc())
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

    let events_modified: Vec<PlantId> = match sqlx::query_as(
        r#"SELECT id, date_created, event_modified FROM plants WHERE event_modified >= $1"#,
    )
    .bind(timestamp.naive_utc())
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
        events_modified: events_modified.iter().map(|item| item.id).collect(),
    };
    Response::new(serde_json::ser::to_string(&response).unwrap().into())
}
