mod api;
mod errors;
mod jsend;
mod models;
mod notification;
mod config;
mod db;
mod schema;
mod core;

use axum::routing::{get, put};
use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse, routing::post, Extension, Router};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tokio::task::JoinHandle;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
// use crate::api::event_handler::{log_stage_complete, start_job};
use crate::api::health_handler::{health_check, health_check2};
use crate::config::{from_env, Config};
use db::connection::{get_connection_pool, PgPool};
use crate::api::config_handler::{create_config_handler, get_config_by_app_and_name_handler, update_config_handler};
// --- 2. Application State ---
// (Shared state accessible by all handlers)


#[derive(Clone)]
pub struct AppState {
    pub config: Config, // Note: Use the type name you have
    pub pool: PgPool,   // Assuming your pool type is PgPool
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

    let pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>> = get_connection_pool(&config.postgres_url).await.unwrap();

    let state = Arc::new(AppState {
        config: config.clone(), // Clone if Config is not already owned/Arc'd
        pool: pool,
    });

    // Build Axum routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/health2", get(health_check2))
        .route("/job-configs", post(create_config_handler))
        .route("/job-configs/{application}/{job_name}", get(get_config_by_app_and_name_handler).put(update_config_handler))
        // .route("/job-runs/start", post(start_job))
        // .route("/job-runs/stage", post(log_stage_complete))
        .with_state(state);

    let app = Router::new().nest("/api", api_routes);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Watchdog service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- 5. Helper Functions ---

/// Checks the `job_runs` table to see if a stage has been completed.

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// --- 6. Error Handling ---
// (A simple error enum for clean handler responses)
