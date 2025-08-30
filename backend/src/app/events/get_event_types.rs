use axum::{
    body::Body,
    extract::{RawPathParams, State},
    http::StatusCode,
    response::Response,
};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use shared::events::{EventDataKind, EventType};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EventTypesRow {
    id: Uuid,
    name: String,
    event_type: Json<EventDataKind>,
}

/// Gets all the event types
pub async fn get_event_types(params: RawPathParams, State(pool): State<PgPool>) -> Response {
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
    let new_plants: Vec<EventTypesRow> =
        match sqlx::query_as(r#"SELECT id, name, event_type FROM plant_event_types"#)
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

    let events: Vec<EventType> = new_plants
        .iter()
        .map(|typ| EventType {
            id: typ.id,
            name: typ.name.clone(),
            kind: typ.event_type.0.clone(),
        })
        .collect();

    let serialize = match serde_json::to_string(&events) {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    Response::new(Body::from(serialize))
}
