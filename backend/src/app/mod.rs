use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::plants::{
        plant::request_plant, plant_demographic::request_plant_demographic,
        plant_new::new_plant, plants_verification::client_verify_plants,
    },
    AppState,
};

mod plants;

pub(super) fn rout_plant() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_plant))
        .route("/get/{id}", get(request_plant))
        .route("/get_demographic/{id}", get(request_plant_demographic))
        .route("/verify-client-list", post(client_verify_plants))
}
