use crate::api::channel_handler::{create_channel_handler, get_all_channel_providers_handler, get_all_channels_handler, get_channel_by_id_handler, update_channel_handler};
use crate::api::config_handler::{create_config_handler, get_all_applications_handler, get_all_configs_handler, get_config_by_app_name_and_job_name_handler, list_jobs_by_app_handler, update_config_handler};
use crate::api::run_handler::{get_run_by_id_handler, trigger_job_handler, update_stage_by_context_handler, update_stage_by_id_handler};

use axum::{
    routing::{get, post},
    Router,
};
use crate::api::health_handler::health_check_handler;
use crate::{SharedState};

pub fn app_routes(state: SharedState) -> Router {
    let channel_routes = Router::new()
        .route("/", get(get_all_channels_handler).post(create_channel_handler))
        .route("/providers", get(get_all_channel_providers_handler))
        .route("/{id}", get(get_channel_by_id_handler).put(update_channel_handler));

    let config_routes = Router::new()
        .route("/", get(get_all_configs_handler).post(create_config_handler))
        .route("/{app_name}", get(list_jobs_by_app_handler))
        .route("/{app_name}/{job_name}", get(get_config_by_app_name_and_job_name_handler).put(update_config_handler));

    let app_job_routes = Router::new()
        .route("/trigger", post(trigger_job_handler))
        .route("/stage-update", post(update_stage_by_context_handler));

    let run_id_routes = Router::new()
        .route("/", get(get_run_by_id_handler))
        .route("/stage-update", post(update_stage_by_id_handler));

    Router::new()
        .route("/health", get(health_check_handler))
        .route("/applications", get(get_all_applications_handler))
        .nest("/channels", channel_routes)
        .nest("/job-configs", config_routes)
        .nest("/applications/{app_name}/jobs/{job_name}", app_job_routes)
        .nest("/job-runs/{job_run_id}", run_id_routes)
        .with_state(state)
}