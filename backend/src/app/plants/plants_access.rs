use axum::{
    extract::{Path, State},
    response::Response,
};
use serde::{Deserialize, Serialize};
use shared::{
    plant::{plant_http::NewPlant, Plant, PlantState},
    HistoryItem,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

pub async fn new_plant(
    State(pool): State<PgPool>,
    axum::Json(new_plant): axum::Json<NewPlant>,
) -> Response {
    let plant_uuid = Uuid::new_v4();
    let result = sqlx::query("INSERT INTO plants(name, id, state) VALUES ($1, $2, $3)")
        .bind(Json(HistoryItem::new_with_timestamp(
            new_plant.name,
            new_plant.timestamp,
        )))
        .bind(plant_uuid.clone())
        .bind(Json(HistoryItem::new_with_timestamp(
            PlantState::Alive,
            new_plant.timestamp,
        )))
        .execute(&pool)
        .await
        .unwrap();

    let new_plant = request_plant(
        Path::from(axum::extract::Path(plant_uuid.to_string())),
        State(pool),
    )
    .await;

    println!("New Plant Registered: {:?}", result);
    new_plant
}

pub async fn request_plant(Path(plant_id): Path<String>, State(pool): State<PgPool>) -> Response {
    let result: PlantDatabase = match sqlx::query_as(&format!(
        "SELECT id, name, state FROM plants where id ='{}'",
        Uuid::parse_str(&plant_id).unwrap()
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(plant) => plant,
        Err(error) => {
            return Response::new(
                serde_json::ser::to_string(&error.to_string())
                    .unwrap()
                    .into(),
            )
        }
    };

    println!("New Plant Registered: {:?}", result);
    let plant = Plant {
        id: result.id,
        name: result.name.0,
        plant_state: result.state.0,
    };
    Response::new(serde_json::ser::to_string(&plant).unwrap().into())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantDatabase {
    pub name: Json<HistoryItem<String>>,
    pub id: Uuid,
    pub state: Json<HistoryItem<PlantState>>,
}
