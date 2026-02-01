mod api;
mod config;
mod core;
mod cron_utils;
mod db;
mod dtos;
mod errors;
mod jsend;
mod models;
mod notification;
mod pubsub;
mod router;
mod scheduler;
mod schema;
mod settings;
mod time_utils;
mod validations;

use crate::config::{Config, from_env};
use crate::db::settings_repository::get_settings;
use crate::models::Settings;
use crate::notification::dispatcher::NotificationDispatcher;
use crate::notification::init::init_notification;
use crate::router::app_routes;
use crate::scheduler::scheduler;
use crate::settings::from_db;
use axum::Router;
use db::connection::{PgPool, get_connection_pool};
use diesel_async::{AsyncConnection, RunQueryDsl, pg::AsyncPgConnection};
use futures::StreamExt;
use serde::Deserialize;
use std::sync::RwLock;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use crate::pubsub::start_listener;

type SharedSettings = Arc<RwLock<Settings>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Config, // Note: Use the type name you have
    pub settings: SharedSettings,
    pub pool: PgPool, // Assuming your pool type is PgPool
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

    let initial_settings = from_db(&pool)
        .await
        .expect("Failed to load initial settings");

    let state = Arc::new(AppState {
        config: config.clone(),
        settings: Arc::new(RwLock::new(initial_settings)),
        pool: pool.clone(),
        dispatcher: dispatcher.clone(),
    });

    // tokio::spawn(scheduler(&get_connection_pool(&config.postgres_url).await.unwrap()));
    let scheduler_pool = pool.clone();
    tokio::spawn(async move { scheduler(&scheduler_pool, &dispatcher, &config).await });

    let pub_sub_pool = pool.clone();
    start_listener(pub_sub_pool, state.clone()).await;

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

// pub async fn start_listener(pool: PgPool, state: SharedState) {
//     tokio::spawn(async move {
//         loop {
//             // We handle the connection result manually because tokio::spawn
//             // usually expects a closure that returns ()
//             let mut connection = match pool.get().await {
//                 Ok(conn) => conn,
//                 Err(e) => {
//                     eprintln!("❌ Failed to get connection from pool: {}", e);
//                     // eprintln!("📡 Listener error: {}. Retrying in 5s...", e);
//                     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//                     continue;
//                 }
//             };
//
//             // Now you can proceed with the listener logic
//             if let Err(e) = diesel::sql_query("LISTEN settings_update")
//                 .execute(&mut connection)
//                 .await
//             {
//                 eprintln!("❌ Failed to LISTEN: {}", e);
//                 return;
//             }
//
//             let stream = connection.notifications_stream();
//             tokio::pin!(stream);
//
//             while let Some(notification_result) = stream.next().await {
//                 match notification_result {
//                     Ok(notification) => {
//                         // DESERIALIZE the JSON payload
//                         let raw_payload = &notification.payload;
//                         match serde_json::from_str::<Settings>(&*raw_payload) {
//                             Ok(data) => {
//                                 println!("🔔 Received {:?} for id: {}", data, data.id);
//                                 {
//                                     let mut config_lock = state.settings.write().expect("Failed to acquire write lock");
//                                     *config_lock = data;
//                                 }
//                                 // Refined Logic: Only update specific cache key or re-fetch row by ID
//                                 // refresh_specific_setting(data.id, pool.clone(), current_settings.clone()).await;
//                             }
//                             Err(e) => eprintln!("❌ Failed to parse JSON payload: {}", e),
//                         }
//                         println!(
//                             "🔔 Settings changed! Channel: {}, Payload: {}",
//                             notification.channel, notification.payload
//                         );
//                         // Your refresh logic goes here
//                     }
//                     Err(e) => {
//                         eprintln!("❌ Notification stream error: {}", e);
//                         break;
//                     }
//                 }
//             }
//         }
//     });
// }
