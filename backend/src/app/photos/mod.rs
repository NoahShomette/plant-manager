use std::fs;

use axum::{Router, routing::post};

use crate::{AppState, app::photos::new_photo::new_photo};

mod get_photo;
mod new_photo;

pub fn route_photos() -> Router<AppState> {
    let _ = fs::create_dir_all("/assets/photos/thumbs");

    Router::new().route("/new", post(new_photo))
    //.route("/get/{id}", get(request_plant))
    //.route("/delete/{id}", get(delete_plant))
}
