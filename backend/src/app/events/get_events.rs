use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::events::{events_http::GetEvent, EventData, EventInstance};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, FromRow)]
pub struct EventInstanceRow {
    pub id: Uuid,
    pub event_type_id: Uuid,
    pub plant_id: Uuid,
    pub data: Json<EventData>,
    pub event_date: NaiveDateTime,
}

/// Gets all the event types
pub async fn get_events(
    State(pool): State<PgPool>,
    axum::Json(request): axum::Json<GetEvent>,
) -> Response {
    let query = match request.request_details {
        shared::events::events_http::GetEventType::Span(naive_date_time, naive_date_time1) => {
            sqlx::query_as(
                r#"SELECT id, event_type_id, plant_id, data, event_date FROM plant_events WHERE event_date >= $1 AND event_date <= $2 AND plant_id = $3 AND event_type_id = $4"#,
            ).bind(naive_date_time).bind(naive_date_time1).bind(request.plant_id).bind(request.event_type)
        }
        shared::events::events_http::GetEventType::LastNth(n) => sqlx::query_as(
            r#"SELECT id, event_type_id, plant_id, data, event_date FROM plant_events WHERE plant_id = $2 AND event_type_id = $3 ORDER BY event_date DESC LIMIT $1"#,
        ).bind(n).bind(request.plant_id).bind(request.event_type),
        shared::events::events_http::GetEventType::All => sqlx::query_as(
            r#"SELECT id, event_type_id, plant_id, data, event_date FROM plant_events WHERE plant_id = $1 AND event_type_id = $2"#,
        ).bind(request.plant_id).bind(request.event_type),
    };

    let new_plants: Vec<EventInstanceRow> = match query.fetch_all(&pool).await {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    let events: Vec<EventInstance> = new_plants
        .iter()
        .map(|typ| EventInstance {
            id: typ.id,
            event_type_id: typ.event_type_id,
            plant_id: typ.plant_id,
            data: typ.data.0.clone(),
            event_date: typ.event_date,
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
