use axum::{Router, routing::get};

use tokio::net::TcpListener;
use tokio::signal;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod static_support;
use static_support::using_serve_dir;

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

    let app = Router::new()
        .route("/health_check", get(health_check_handler))
        .route(
            "/",
            get(db::postgres::using_connection_pool_extractor)
                .post(db::postgres::using_connection_pool_extractor),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());

    tokio::join!(serve(using_serve_dir(), 3001), serve(app, 8080));
}

async fn health_check_handler() -> String {
    "Positive Health Check".to_string()
}

async fn serve(app: Router, port: u16) {
    let addr_str = format!("[::]:{}", port);
    tracing::info!("listening on {}", addr_str);
    let listener = TcpListener::bind(addr_str.as_str())
        .await
        .expect("failed to bind");
    axum::serve(
        listener,
        app.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

// graceful shutdown
async fn shutdown_signal() {
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

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
