use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    events::{events_http::GetEventType, CustomEnum, EventData, PLANT_NAME_EVENT_ID, PLANT_STATE_ID},
    plant::{Plant, PlantDemographic, PlantState},
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::{uuid, Uuid};

use crate::app::events::{get_event_custom, get_last_event};

/// Struct which represents an entire plant
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantDatabase {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub event_modified: NaiveDateTime,
}

pub async fn request_plant_demographic(
    Path(plant_id): Path<String>,
    State(pool): State<PgPool>,
) -> Response {
    let plant_id = match Uuid::parse_str(&plant_id) {
        Ok(result) => result,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };
    let result: PlantDatabase = match sqlx::query_as(&format!(
        "SELECT id, date_created, event_modified FROM plants where id ='{}'",
        plant_id
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(plant) => plant,
        Err(error) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error.to_string()))
                .unwrap();
        }
    };

    let name_event = match get_last_event(uuid!(PLANT_NAME_EVENT_ID), plant_id, pool.clone()).await {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };
    let state_event = match get_last_event(uuid!(PLANT_STATE_ID), plant_id, pool).await {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    let name = match name_event {
        Some(event) => {
            let EventData::String(name) = event.data else {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(
                        "Name Event Instance is the wrong type".to_string(),
                    ))
                    .unwrap();
            };
            Some(name)
        }
        None => None,
    };

    let state = match state_event {
        Some(event) => {
            let EventData::CustomEnum(state) = event.data else {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(
                        "Name Event Instance is the wrong type".to_string(),
                    ))
                    .unwrap();
            };
            state
        }
        None => CustomEnum::plant_state(),
    };

    let plant: PlantDemographic = PlantDemographic::from_plant(
        Plant {
            id: result.id,
            date_created: result.date_created,
            event_modified: result.event_modified,
        },
        name,
        state,
    );
    Response::new(serde_json::ser::to_string(&plant).unwrap().into())
}
