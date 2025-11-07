mod api;
mod db;
mod errors;
mod jsend;
mod models;
mod notification;

use crate::api::config_handler::create_config;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_dynamodb::config::{Credentials, SharedCredentialsProvider};
// use aws_credential_types::provider::SharedCredentialsProvider;
// use aws_credential_types::Credentials;
use aws_sdk_dynamodb::{
    Client as DynamoClient,
    types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
};
use axum::routing::get;
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_item};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::task::JoinHandle;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;
// use crate::api::event_handler::{log_stage_complete, start_job};
use crate::api::health_handler::{health_check, health_check2};
use crate::db::setup_dynamodb_tables;

// --- 2. Application State ---
// (Shared state accessible by all handlers)

// We use Arc (Atomic Reference Counting) to share state safely across threads.
// The DashMap will store: run_id -> Vec<JoinHandle>
// This lets us track and cancel the alert tasks for a specific run.
type ActiveTasks = Arc<DashMap<String, Vec<JoinHandle<()>>>>;

#[derive(Clone)]
struct AppState {
    db: DynamoClient,
    active_tasks: ActiveTasks,
    config_table: String,
    run_table: String,
}

// --- 3. Main Application ---

#[tokio::main]
async fn main() {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Load AWS config
    // let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    // Get the config loader, but don't `await` it yet
    let config_loader = aws_config::load_defaults(BehaviorVersion::latest());

    // Now, modify the loader to point to your local endpoint
    // let sdk_config = config_loader
    //     .region(Region::new("us-east-1")) // Any region works for local
    //     .endpoint_url("http://localhost:8000") // This is the magic line
    //     .load()
    //     .await;

    // You can create the credentials first for clarity

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        .region(Region::new("us-east-1")) // Any region works for local
        // DynamoDB run locally uses port 8000 by default.
        .endpoint_url("http://localhost:8000")
        .load()
        .await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config).build();

    let db_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local_config);


    // let static_creds = Credentials::new(
    //     "DUMMY_KEY",
    //     "DUMMY_SECRET",
    //     None,
    //     None,
    //     "local",
    // );
    //
    // let sdk_config = SdkConfig::builder()
    //     .behavior_version(BehaviorVersion::latest())
    //     .region(Region::new("us-east-1"))
    //     .endpoint_url("http://localhost:8000") // Point to local DynamoDB
    //     .credentials_provider(SharedCredentialsProvider::new(static_creds))
    //     .build();
    // let db_client = DynamoClient::new(&sdk_config);

    // Define table names
    let config_table = "job_configs".to_string();
    let run_table = "job_runs".to_string();

    // --- One-time setup: Create tables (for local testing) ---
    // In production, you'd use IaC (Terraform, CloudFormation)
    setup_dynamodb_tables(&db_client, &config_table, &run_table)
        .await
        .expect("Failed to create DynamoDB tables");
    // --- End setup ---

    // Initialize shared state
    let shared_state = AppState {
        db: db_client,
        active_tasks: Arc::new(DashMap::new()),
        config_table,
        run_table,
    };

    // Build Axum routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health2", get(health_check2))
        .route("/config", post(create_config))
        // .route("/event/start", post(start_job))
        // .route("/event/stage", post(log_stage_complete))
        .with_state(shared_state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Alert service listening on {}", addr);
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
