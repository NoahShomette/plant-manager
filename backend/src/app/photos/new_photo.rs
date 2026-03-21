use std::io::Cursor;

use crate::app::events::new_event;
use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
use chrono::Utc;
use image::{DynamicImage, ImageDecoder, ImageReader, codecs::png::PngDecoder};
use shared::{
    DirtyCache,
    events::{PHOTO_EVENT_TYPE_ID, events_http::NewEvent},
    photos::NewPhoto,
};
use sqlx::PgPool;
use tokio::{fs, sync::mpsc::Sender};
use tracing::debug;
use uuid::Uuid;
use uuid::uuid;

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn new_photo(
    State(pool): State<PgPool>,
    State(dirt_cache): State<Sender<DirtyCache>>,
    axum::Json(new_photo): axum::Json<NewPhoto>,
) -> Response {
    let photo_id = Uuid::new_v4();
    let file_location = format!("./assets/photos/{}.png", photo_id);
    let thumbs_file_location = format!("./assets/photos/thumbs/{}.png", photo_id);

    let thumbnail =
        match ImageReader::new(Cursor::new(new_photo.photo_binary.clone())).with_guessed_format() {
            Ok(ok) => match ok.decode() {
                Ok(ok) => ok,
                Err(err) => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(err.to_string()))
                        .unwrap();
                }
            },
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(err.to_string()))
                    .unwrap();
            }
        };

    let thumbnail = thumbnail.resize(200, 200, image::imageops::FilterType::Gaussian);

    let mut buf: Vec<u8> = Vec::new();
    thumbnail
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();

    let _result = match fs::write(&thumbs_file_location, buf).await {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    let _result = match fs::write(&file_location, new_photo.photo_binary).await {
        Ok(ok) => ok,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_string()))
                .unwrap();
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
                    .unwrap();
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
