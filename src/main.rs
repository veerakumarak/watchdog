mod api;
mod errors;
mod jsend;
mod models;
mod config;
mod db;
mod schema;
mod core;
mod dtos;
mod cron_utils;
mod scheduler;
mod time_utils;
mod notification;

use axum::routing::{get};
use axum::{routing::post, Router};
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::api::health_handler::{health_check_handler};
use crate::config::{from_env, Config};
use db::connection::{get_connection_pool, PgPool};
use crate::api::config_handler::{create_config_handler, get_all_applications_handler, get_all_configs_handler, get_config_by_app_name_and_job_name_handler, list_jobs_by_app_handler, update_config_handler};
use crate::api::run_handler::{get_run_by_id_handler, job_run_complete_with_run_id_handler, job_run_complete_without_run_id_handler, job_run_failed_with_run_id_handler, job_run_failed_without_run_id_handler, job_run_start_with_run_id_handler, job_run_start_without_run_id_handler, job_run_trigger_handler};
use crate::notification::dispatcher::NotificationDispatcher;
use crate::notification::init::init_notification;
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
        scheduler(&scheduler_pool, &dispatcher).await
    });

    let web_build_path = "./web/dist";
    let serve_dir = ServeDir::new(web_build_path)
        .not_found_service(ServeFile::new(format!("{}/index.html", web_build_path)));

    // Build Axum routes
    let api_routes = Router::new()
        .route("/health", get(health_check_handler))
        .route("/applications", get(get_all_applications_handler))
        .route("/applications/{app_name}/job-configs", get(list_jobs_by_app_handler))
        .route("/job-configs", get(get_all_configs_handler).post(create_config_handler))
        .route("/job-configs/{app_name}/{job_name}", get(get_config_by_app_name_and_job_name_handler).put(update_config_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/trigger", post(job_run_trigger_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{job_run_id}/{stage_name}/start", post(job_run_start_with_run_id_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{job_run_id}/{stage_name}/complete", post(job_run_complete_with_run_id_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{job_run_id}/{stage_name}/failed", post(job_run_failed_with_run_id_handler))
        .route("/job-runs/{job_run_id}", get(get_run_by_id_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{stage_name}/start", post(job_run_start_without_run_id_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{stage_name}/complete", post(job_run_complete_without_run_id_handler))
        .route("/job-runs/{app_name}/job-runs/{job_name}/{stage_name}/failed", post(job_run_failed_without_run_id_handler))
        .fallback_service(serve_dir)
        .with_state(state);

    let app = Router::new().nest("/api", api_routes);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Watchdog service listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
