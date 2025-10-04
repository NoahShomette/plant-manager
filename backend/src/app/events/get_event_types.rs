use axum::{
    body::Body,
    extract::{RawPathParams, State},
    http::StatusCode,
    response::Response,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::events::{EventDataKind, EventType};
use sqlx::{prelude::FromRow, types::Json, PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EventTypesRow {
    id: Uuid,
    name: String,
    event_type: Json<EventDataKind>,
    /// Is this event type deletable by the user
    deletable: bool,
    /// Is this even type modifiable by the user
    modifiable: bool,
    /// Is this event type unique - there can be only one event type
    is_unique: bool,
}

pub enum GetDatabaseEventTypes {
    All(DateTime<Utc>),
    Type(Uuid),
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
    let events: Vec<EventType> =
        match get_event_types_custom(GetDatabaseEventTypes::All(timestamp), pool).await {
            Ok(result) => result,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(err.to_string()))
                    .unwrap();
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

/// Gets all the event types
pub async fn get_event_types_custom(
    event_type: GetDatabaseEventTypes,
    pool: Pool<Postgres>,
) -> Result<Vec<EventType>, sqlx::Error> {
    let events: Vec<EventTypesRow> = match event_type{

        GetDatabaseEventTypes::All(timestamp) => {
            sqlx::query_as(
        r#"SELECT id, name, event_type, deletable, modifiable, is_unique FROM event_types"#,
    )
    .fetch_all(&pool)
    .await?
        },

        GetDatabaseEventTypes::Type(uuid) => {

             sqlx::query_as(
        r#"SELECT id, name, event_type, deletable, modifiable, is_unique FROM event_types WHERE id = $1"#,
    ).bind(uuid)
    .fetch_all(&pool)
    .await?

        },
    };

    let events: Vec<EventType> = events
        .iter()
        .map(|typ| EventType {
            id: typ.id,
            name: typ.name.clone(),
            kind: typ.event_type.0.clone(),
            deletable: typ.deletable,
            modifiable: typ.modifiable,
            is_unique: typ.is_unique,
        })
        .collect();

    Ok(events)
}

/// Gets all the event types
pub async fn get_event_type_single(
    event_type: Uuid,
    pool: Pool<Postgres>,
) -> Result<EventType, sqlx::Error> {
    let events: EventTypesRow = 
             sqlx::query_as(
        r#"SELECT id, name, event_type, deletable, modifiable, is_unique FROM event_types WHERE id = $1"#,
    ).bind(event_type)
    .fetch_one(&pool)
    .await?;

    let events: EventType =  EventType {
            id: events.id,
            name: events.name.clone(),
            kind: events.event_type.0.clone(),
            deletable: events.deletable,
            modifiable: events.modifiable,
            is_unique: events.is_unique,
        };

    Ok( events)
}
