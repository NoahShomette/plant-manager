use axum::{extract::FromRef, routing::get, Router};

use futures_util::lock::Mutex;
use shared::DirtyCache;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self, Sender};
use tokio::time::sleep;
use tokio::{net::TcpListener, sync::mpsc::Receiver};
use tokio::{select, signal, spawn};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::io::Error;
use std::{sync::Arc, time::Duration};

mod app;
mod static_support;
use static_support::using_serve_dir;

use crate::app::{dirty_cache_sse_handler, rout_event, rout_plant};

// the application state
#[derive(Clone)]
struct AppState {
    // that holds some api specific state
    pub database_pools: Pool<Postgres>,
    pub dirty_cache_sender: Sender<DirtyCache>,
    pub dirty_cache_receiver: Arc<Mutex<Receiver<DirtyCache>>>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(app_state: &AppState) -> Pool<Postgres> {
        app_state.database_pools.clone()
    }
}

impl FromRef<AppState> for Sender<DirtyCache> {
    fn from_ref(app_state: &AppState) -> Sender<DirtyCache> {
        app_state.dirty_cache_sender.clone()
    }
}

impl FromRef<AppState> for Arc<Mutex<Receiver<DirtyCache>>> {
    fn from_ref(app_state: &AppState) -> Arc<Mutex<Receiver<DirtyCache>>> {
        app_state.dirty_cache_receiver.clone()
    }
}

#[tokio::main]
async fn main() {
    println!("reached main");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    tracing::info!("connecting to database: {}", db_connection_str);
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Successfuly Migrated Database"),
        Err(failed_migration) => panic!(
            "Failed to migrate database with error: {}",
            failed_migration
        ),
    }
    let (sender, receiver) = mpsc::channel(250);

    let state = AppState {
        database_pools: pool,
        dirty_cache_sender: sender,
        dirty_cache_receiver: Arc::new(Mutex::new(receiver)),
    };
    let app: Router = Router::new()
        .merge(rout_main())
        .nest("/plants", rout_plant())
        .nest("/events", rout_event())
        .route("/dirty-cache", get(dirty_cache_sse_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let _ = tokio::join!(serve(using_serve_dir(), 3001), serve(app, 8080));
}

fn rout_main() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check_handler))
        .route(
            "/",
            get(db::postgres::using_connection_pool_extractor)
                .post(db::postgres::using_connection_pool_extractor),
        )
}

async fn health_check_handler() -> String {
    "Positive Health Check".to_string()
}

async fn serve(app: Router, port: u16) -> Result<(), Error> {
    let shutdown_signal = shutdown_signal(Duration::from_secs(1)).await;
    let mut server_shutdown_signal = shutdown_signal.subscribe();
    let addr_str = format!("[::]:{}", port);
    tracing::info!("listening on {}", addr_str);
    let listener = TcpListener::bind(addr_str.as_str())
        .await
        .expect("failed to bind");

    let server = axum::serve(
        listener,
        app.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .with_graceful_shutdown(async move {
        let _ = server_shutdown_signal.recv().await;
    });

    let mut outer_shutdown_signal = shutdown_signal.subscribe();

    select! {
        // Server shutdown by itself.
        res = server => {
            if let Err(err) = res {
                tracing::error!("Server error: {err:?}");
                return Err(err.into());
            }
        },
        // Hard shutdown.
        _ = async {
            loop {
                if let Ok(Shutdown::Hard) = outer_shutdown_signal.recv().await {
                    break;
                }
            }
        } => {},
    }
    Ok(())
}

#[derive(Copy, Clone, Debug)]

enum Shutdown {
    /// Graceful shutdown, finish handling in-flight requests.
    Graceful,

    /// Hard shutdown, abort immediately.
    Hard,
}

// graceful shutdown
async fn shutdown_signal(grace_period: Duration) -> broadcast::Sender<Shutdown> {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let (tx, _rx) = broadcast::channel(2);
    let sender = tx.clone();
    spawn(async move {
        select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        info!("Initiating graceful shutdown");

        sender.send(Shutdown::Graceful).unwrap();

        sleep(grace_period).await;

        info!("Grace period elapsed, shutting down hard");

        sender.send(Shutdown::Hard).unwrap();
    });
    tx
}
