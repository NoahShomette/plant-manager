use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::{
    events::get_event_types_custom, plants::get_demographic::request_plant_demographic,
};

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn delete_plant(
    Path(plant_id): Path<String>,
    State(pool): State<PgPool>
) -> Response {
    let plant_uuid = Uuid::new_v4();

    let event_types = match get_event_types_custom(
        crate::app::events::GetDatabaseEventTypes::All(Utc::now()),
        pool.clone(),
    )
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

    let result =
        sqlx::query("INSERT INTO plants(id, date_created, event_modified) VALUES ($1, $2, $2)")
            .bind(plant_uuid.clone())
            .bind(Utc::now().naive_utc())
            .execute(&pool)
            .await
            .unwrap();

    let new_plant = request_plant_demographic(
        Path::from(axum::extract::Path(plant_uuid.to_string())),
        State(pool),
    )
    .await;

    println!("New Plant Registered: {:?}", result);
    new_plant
}
