use axum::{
    extract::{Path, State},
    response::Response,
};
use shared::plant::{
    plant_http::{NewPlant, PlantResponse, RequestPlant},
    PlantState,
};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

pub async fn new_plant(
    State(pool): State<PgPool>,
    axum::Json(new_plant): axum::Json<NewPlant>,
) -> Response {
    let plant_uuid = Uuid::new_v4();
    let result = sqlx::query("INSERT INTO plants(name, id, state) VALUES ($1, $2, $3)")
        .bind(new_plant.name)
        .bind(plant_uuid.clone())
        .bind(Json(PlantState::Alive))
        .execute(&pool)
        .await
        .unwrap();

    println!("New Plant Registered: {:?}", result);
    Response::new(plant_uuid.to_string().into())
}

pub async fn request_plant(Path(plant_id): Path<String>, State(pool): State<PgPool>) -> Response {
    let result: PlantResponse = sqlx::query_as(&format!(
        "SELECT id, name, state FROM plants where id ='{}'",
        Uuid::parse_str(&plant_id).unwrap()
    ))
    .fetch_one(&pool)
    .await
    .unwrap();

    println!("New Plant Registered: {:?}", result);
    Response::new(serde_json::ser::to_string(&result).unwrap().into())
}
