use crate::app::events::new_event;
use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::Response,
    Json,
};
use chrono::Utc;
use shared::{
    events::{events_http::NewEvent, PHOTO_EVENT_TYPE_ID},
    photos::NewPhoto,
    DirtyCache,
};
use sqlx::PgPool;
use tokio::{fs, sync::mpsc::Sender};
use tracing::debug;
use uuid::uuid;
use uuid::Uuid;

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn new_photo(
    State(pool): State<PgPool>,
    State(dirt_cache): State<Sender<DirtyCache>>,
    axum::Json(new_photo): axum::Json<NewPhoto>,
) -> Response {
    let photo_id = Uuid::new_v4();
    let file_location = format!("./assets/photos/{}.png", photo_id);

    let _result = match fs::write(&file_location, new_photo.photo_binary).await {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap()
        }
    };

    let _result =
        match sqlx::query("INSERT INTO photos(id, file_location, photo_date) VALUES ($1, $2, $3)")
            .bind(photo_id.clone())
            .bind(&file_location)
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

    let new_photo_event_row = new_event(
        State(pool.clone()),
        State(dirt_cache.clone()),
        Json(NewEvent {
            event_type: uuid!(PHOTO_EVENT_TYPE_ID),
            plant_id: new_photo.plant_id,
            event_data: shared::events::EventData::String(file_location),
            event_date: Utc::now().naive_utc(),
        }),
    )
    .await;
    debug!("RESULT: {:?}", new_photo_event_row);

    let _ = dirt_cache
        .send(DirtyCache {
            cache: shared::CacheType::Plant(new_photo.plant_id),
        })
        .await;



    new_photo_event_row
}
