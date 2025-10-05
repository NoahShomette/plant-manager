use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    events::{events_http::NewEvent, EventData, EventDataKind, EventInstance},
    DirtyCache,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EventTypesRow {
    event_type: Json<EventDataKind>,
    is_unique: bool,
    id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, FromRow)]
pub struct EventInstanceRow {
    pub id: Uuid,
    pub event_type_id: Uuid,
    pub plant_id: Uuid,
    pub data: Json<EventData>,
    pub event_date: NaiveDateTime,
}

/// Gets all the event types
pub async fn new_event(
    State(pool): State<PgPool>,
    State(dirt_cache): State<Sender<DirtyCache>>,
    axum::Json(new_event): axum::Json<NewEvent>,
) -> Response {
    let event_type: EventTypesRow =
        match sqlx::query_as(r#"SELECT id, event_type, is_unique FROM event_types where id = $1"#)
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
    
    let query_string = match event_type.is_unique {
        true => format!(
            r#"INSERT INTO events_unique(id, event_type_id, plant_id, data, event_date) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (event_type_id, plant_id) DO UPDATE SET event_date = EXCLUDED.event_date, data = EXCLUDED.data RETURNING *"#
        ),
        false => format!(
            r#"INSERT INTO events(id, event_type_id, plant_id, data, event_date) VALUES ($1, $2, $3, $4, $5) RETURNING *"#
        ),
    };

    let result: EventInstanceRow = match sqlx::query_as(&query_string)
        .bind(Uuid::new_v4())
        .bind(new_event.event_type)
        .bind(new_event.plant_id)
        .bind(Json(new_event.event_data))
        .bind(new_event.event_date)
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

    let new_event_instance = EventInstance {
        id: result.id,
        event_type_id: result.event_type_id,
        plant_id: result.plant_id,
        data: result.data.0,
        event_date: result.event_date,
    };

    let _ = dirt_cache
        .send(DirtyCache {
            cache: shared::CacheType::Event(new_event.plant_id, event_type.id, result.event_date),
        })
        .await;

    let serialize = match serde_json::to_string(&new_event_instance) {
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
        .body(Body::from(serialize))
        .unwrap();
}
