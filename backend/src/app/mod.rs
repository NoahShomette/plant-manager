mod events;
mod plants;

use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
pub use events::rout_event;
use futures_util::{lock::Mutex, stream::Stream};
use shared::DirtyCache;
use tokio::sync::mpsc::Receiver;

pub use plants::rout_plant;

pub async fn dirty_cache_sse_handler(
    State(receiver): State<Arc<Mutex<Receiver<DirtyCache>>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        let mut receiver = receiver.lock().await;
        loop {
            if let Some(event) = receiver.recv().await{
                yield Ok(Event::default().data(serde_json::to_string(&event).unwrap()));
            }
        }
    };
    Sse::new(stream).keep_alive(KeepAlive::default())
}
