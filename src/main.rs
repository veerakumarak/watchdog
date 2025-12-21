mod api;
mod errors;
mod jsend;
mod models;
mod config;
mod db;
mod schema;
mod core;
mod cron_utils;
mod scheduler;
mod time_utils;
mod notification;
mod validations;
mod dtos;
mod router;

use axum::{Router};
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::config::{from_env, Config};
use db::connection::{get_connection_pool, PgPool};
use crate::notification::dispatcher::NotificationDispatcher;
use crate::notification::init::init_notification;
use crate::router::app_routes;
use crate::scheduler::scheduler;

#[derive(Clone)]
pub struct AppState {
    pub config: Config, // Note: Use the type name you have
    pub pool: PgPool,   // Assuming your pool type is PgPool
    pub dispatcher: NotificationDispatcher,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = from_env();

    let pool: PgPool = get_connection_pool(&config.postgres_url)
        .await
        .expect("Failed to create Postgres connection pool! Is the DB running?");

    let dispatcher = init_notification(pool.clone()).await;

    let state = Arc::new(AppState {
        config: config.clone(),
        pool: pool.clone(),
        dispatcher: dispatcher.clone(),
    });

    // tokio::spawn(scheduler(&get_connection_pool(&config.postgres_url).await.unwrap()));
    let scheduler_pool = pool.clone();
    tokio::spawn(async move {
        scheduler(&scheduler_pool, &dispatcher, &config).await
    });

    let web_build_path = "./web/dist";
    let serve_dir = ServeDir::new(web_build_path)
        .not_found_service(ServeFile::new(format!("{}/index.html", web_build_path)));

    // Build Axum routes
    let api_routes = app_routes(state);

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(serve_dir);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Watchdog service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
