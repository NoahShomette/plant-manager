use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use shared::{
    plant::{plant_http::ModifyPlant, PlantState},
    InfallibleHistoryItem,
};
use sqlx::{prelude::FromRow, types::Json, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantNameRow {
    pub name: Json<InfallibleHistoryItem<String>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlantStateRow {
    pub state: Json<InfallibleHistoryItem<PlantState>>,
}

/// Creates a new plant on the server and returns a basic plant demographic to the client
pub async fn modify_plant(
    Path(plant_id): Path<String>,
    State(pool): State<PgPool>,
    axum::Json(modify_plant): axum::Json<ModifyPlant>,
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
    match modify_plant {
        ModifyPlant::ChangeName(new_name) => {
            let new_plants: PlantNameRow =
                match sqlx::query_as(r#"SELECT name FROM plants WHERE id = $1"#)
                    .bind(plant_id.clone())
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

            let mut name = new_plants.name.0;
            name.insert(new_name);

            let result =
                match sqlx::query("UPDATE plants SET name = $1, last_modified = $3 WHERE id = $2")
                    .bind(Json(name))
                    .bind(plant_id)
                    .bind(Utc::now().naive_utc())
                    .execute(&pool)
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
        }
        ModifyPlant::ChangeState(plant_state) => {
            let new_plants: PlantStateRow =
                match sqlx::query_as(r#"SELECT state FROM plants WHERE id = $1"#)
                    .bind(plant_id.clone())
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

            let mut state = new_plants.state.0;
            state
                .item
                .item
                .insert(Utc::now().naive_utc().and_utc().timestamp(), plant_state);

            let result =
                match sqlx::query("UPDATE plants SET state = $1, last_modified = $3 WHERE id = $2")
                    .bind(Json(state))
                    .bind(plant_id)
                    .bind(Utc::now().naive_utc())
                    .execute(&pool)
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
        }
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("".to_string()))
        .unwrap()
}
