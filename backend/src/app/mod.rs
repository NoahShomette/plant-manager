use axum::Router;

use crate::AppState;

mod plants;

pub(super) fn rout_plant() -> Router<AppState> {
    Router::new()
}
