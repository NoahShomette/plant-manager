use axum::{routing::{get, post}, Router};

use crate::{app::photos::new_photo::new_photo, AppState};

mod new_photo;
mod get_photo;

pub fn route_photos() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_photo))
        //.route("/get/{id}", get(request_plant))
        //.route("/delete/{id}", get(delete_plant))
}
