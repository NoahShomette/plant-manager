use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::plants::{
        delete::delete_plant, get_demographic::request_plant_demographic,
        get_full_plant::request_plant, get_plant_list::get_plant_list, plant_new::new_plant,
    },
    AppState,
};

pub mod delete;
/// Module for creating plant demographics from the database
pub mod get_demographic;
/// Module for creating full Plant instances from the database
pub mod get_full_plant;
pub mod get_plant_list;
pub mod plant_new;

pub fn rout_plant() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_plant))
        .route("/get/{id}", get(request_plant))
        .route("/delete/{id}", get(delete_plant))
        .route("/get-demographic/{id}", get(request_plant_demographic))
        .route("/get-plant-list/{timestamp}", get(get_plant_list))
}
