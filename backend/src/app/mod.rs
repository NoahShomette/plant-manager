use axum::{routing::{get, post}, Router};

use crate::{app::plants::plants_access::{new_plant, request_plant}, AppState};

mod plants;

pub(super) fn rout_plant() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_plant))
        .route("/get/{id}", get(request_plant))
}
