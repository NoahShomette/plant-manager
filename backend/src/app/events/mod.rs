use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

pub use get_event_types::{get_event_types_custom, GetDatabaseEventTypes};
pub use get_events::{get_event_custom, get_last_event};
pub use new_event::new_event;

mod get_event_types;
mod get_events;
mod new_event;
mod new_event_type;

pub fn rout_event() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_event::new_event))
        .route("/new-type", post(new_event_type::new_event_type))
        .route(
            "/get-types/{timestamp}",
            get(get_event_types::get_event_types),
        )
        .route("/get-events", post(get_events::get_events))
}
