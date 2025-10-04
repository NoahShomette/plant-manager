use crate::app::{
    events::{get_event_types_custom, new_event},
    plants::get_demographic::request_plant_demographic,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use chrono::Utc;
use shared::{
    events::{events_http::NewEvent, CustomEnum, PLANT_NAME_EVENT_ID, PLANT_STATE_ID},
    plant::plant_http::NewPlant,
    DirtyCache,
};
use sqlx::PgPool;
use tokio::sync::mpsc::Sender;
use tracing::debug;
use uuid::uuid;
use uuid::Uuid;

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn new_plant(
    State(pool): State<PgPool>,
    State(dirt_cache): State<Sender<DirtyCache>>,
    axum::Json(new_plant): axum::Json<NewPlant>,
) -> Response {
    let plant_uuid = Uuid::new_v4();

    let result = match sqlx::query(
        "INSERT INTO plants(id, date_created, event_modified) VALUES ($1, $2, $2)",
    )
    .bind(plant_uuid.clone())
    .bind(Utc::now().naive_utc())
    .execute(&pool)
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

    let result = new_event(
        State(pool.clone()),
        State(dirt_cache.clone()),
        Json(NewEvent {
            event_type: uuid!(PLANT_NAME_EVENT_ID),
            plant_id: plant_uuid,
            event_data: shared::events::EventData::String(new_plant.name),
            event_date: Utc::now().naive_utc(),
        }),
    )
    .await;
    debug!("RESULT: {:?}", result);

    let _ = new_event(
        State(pool.clone()),
        State(dirt_cache.clone()),
        Json(NewEvent {
            event_type: uuid!(PLANT_STATE_ID),
            plant_id: plant_uuid,
            event_data: shared::events::EventData::CustomEnum(CustomEnum::plant_state()),
            event_date: Utc::now().naive_utc(),
        }),
    )
    .await;

    let _ = dirt_cache
        .send(DirtyCache {
            cache: shared::CacheType::Plant(plant_uuid),
        })
        .await;

    let new_plant = request_plant_demographic(
        Path::from(axum::extract::Path(plant_uuid.to_string())),
        State(pool),
    )
    .await;

    println!("New Plant Registered: {:?}", result);
    new_plant
}
