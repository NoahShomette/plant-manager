use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::events::{
    events_http::{GetEvent, GetEventType},
    EventData, EventInstance, EventType,
};
use sqlx::{prelude::FromRow, types::Json, PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::app::events::get_event_types::{get_event_types_custom, GetDatabaseEventTypes};

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
    let events = match get_event_custom(
        request.event_type,
        request.plant_id,
        request.request_details,
        pool,
    )
    .await
    {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

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

/// Gets an event thats unique
pub async fn get_last_event(
    event_type: Uuid,
    plant_id: Uuid,
    pool: Pool<Postgres>,
) -> Result<Option<EventInstance>, sqlx::Error> {
    let events = get_event_custom(event_type, plant_id, GetEventType::LastNth(1), pool).await?;

    Ok(events.get(0).cloned())
}

/// Gets events from the given
pub async fn get_event_custom(
    event_type: Uuid,
    plant_id: Uuid,
    request_details: GetEventType,
    pool: Pool<Postgres>,
) -> Result<Vec<EventInstance>, sqlx::Error> {
    let event_types: Vec<EventType> =
        get_event_types_custom(GetDatabaseEventTypes::Type(event_type), pool.clone()).await?;

    let event_type = event_types
        .get(0)
        .ok_or(sqlx::Error::InvalidArgument(String::from(
            "Event Type not found",
        )))?;

    let table_name = event_type.table_name();

    // Returning events Path
    // Get event type
    // If the event type is unique run query on only the unique events table.
    // If the event type is not unique then run query on the main table.
    // Check if we have any results. If we have none check if we are an infallible event.
    // If we are an infallible event and we have no results in the main table then pull from the unique table

    let mut query_string: String = "".to_string();

    // Query the basic tables, either the unique events if thats the event is unique or the main table for all others
    let query = match request_details {
        shared::events::events_http::GetEventType::Span(naive_date_time, naive_date_time1) => {
            query_string = format!(
                r#"SELECT id, event_type_id, plant_id, data, event_date FROM {} WHERE event_date >= $1 AND event_date <= $2 AND plant_id = $3 AND event_type_id = $4"#,
                table_name
            );

            sqlx::query_as(&query_string)
                .bind(naive_date_time)
                .bind(naive_date_time1)
                .bind(plant_id)
                .bind(event_type.id)
        }
        shared::events::events_http::GetEventType::LastNth(n) => {
            query_string = format!(
                r#"SELECT id, event_type_id, plant_id, data, event_date FROM {} WHERE plant_id = $2 AND event_type_id = $3 ORDER BY event_date DESC LIMIT $1"#,
                table_name
            );
            sqlx::query_as(&query_string)
                .bind(n)
                .bind(plant_id)
                .bind(event_type.id)
        }
        shared::events::events_http::GetEventType::All => {
            query_string = format!(
                r#"SELECT id, event_type_id, plant_id, data, event_date FROM {} WHERE plant_id = $1 AND event_type_id = $2"#,
                table_name
            );

            sqlx::query_as(&query_string)
                .bind(plant_id)
                .bind(event_type.id)
                .bind(table_name)
        }
    };

    let new_plants: Vec<EventInstanceRow> = query.fetch_all(&pool).await?;

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
    Ok(events)
}
