use std::fs;

use axum::{
    routing::post,
    Router,
};

use crate::{app::photos::new_photo::new_photo, AppState};

mod get_photo;
mod new_photo;

pub fn route_photos() -> Router<AppState> {
    let _ = fs::create_dir_all("/assets/photos");

    Router::new().route("/new", post(new_photo))
    //.route("/get/{id}", get(request_plant))
    //.route("/delete/{id}", get(delete_plant))
}
