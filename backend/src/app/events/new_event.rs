use axum::{
    body::Body,
    extract::{RawPathParams, State},
    http::StatusCode,
    response::Response,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::events::{events_http::NewEvent, EventDataKind, EventType};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EventTypesRow {
    event_type: Json<EventDataKind>,
}

/// Gets all the event types
pub async fn new_event(
    State(pool): State<PgPool>,
    axum::Json(new_event): axum::Json<NewEvent>,
) -> Response {
    let event_type: EventTypesRow =
        match sqlx::query_as(r#"SELECT event_type FROM plant_event_types where id = $1"#)
            .bind(new_event.event_type)
            .fetch_one(&pool)
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

    if !new_event.event_data.equals_kind(event_type.event_type.0) {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(
                "Event Type sent does not match event type of event".to_string(),
            ))
            .unwrap();
    };

    let result = match sqlx::query("INSERT INTO plant_events(id, event_type_id, plant_id, data, date_created) VALUES ($1, $2, $3, $4, $5)")
    .bind(Uuid::new_v4())
    .bind(new_event.event_type)
    .bind(new_event.plant_id)
    .bind(Json(new_event.event_data))
    .bind(Utc::now().naive_utc())
    .execute(&pool)
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
    return Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("".to_string()))
        .unwrap();
}
