use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::plants::{
        get_demographic::request_plant_demographic, get_plant_list::get_plant_demographics,
        get_full_plant::request_plant, modify_plant::modify_plant, plant_new::new_plant,
    },
    AppState,
};

/// Module for creating plant demographics from the database
pub mod get_demographic;
pub mod get_plant_list;
/// Module for creating full Plant instances from the database
pub mod get_full_plant;
pub mod modify_plant;
pub mod plant_new;

pub fn rout_plant() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_plant))
        .route("/modify/{id}", post(modify_plant))
        .route("/get/{id}", get(request_plant))
        .route("/get-demographic/{id}", get(request_plant_demographic))
        .route("/get-plant-list/{timestamp}", get(get_plant_demographics))
}
