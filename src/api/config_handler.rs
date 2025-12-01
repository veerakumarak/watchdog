use axum::extract::{Path, State};
use axum::Json;
use tracing::info;
use crate::{SharedState};
use crate::db::config_repository::{get_job_config_by_application_and_name, insert_config, save_config};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{JobConfig, NewJobConfig};

pub async fn get_config_by_app_and_name_handler(
    State(state): State<SharedState>,
    Path((application, job_name)): Path<(String, String)>,
) -> Result<AppResponse<JobConfig>, AppError> {
    let mut conn = state.pool.get().await?;

    let job_config = get_job_config_by_application_and_name(&mut conn, &application, &job_name).await?;

    if let Some(_job_config) = job_config {
        Ok(AppResponse::success_one("config", _job_config))
    } else {
        Err(AppError::NotFound(format!("Configuration already exists for application '{}' and job '{}'", application, job_name)))
    }
}

pub async fn create_config_handler(
    State(state): State<SharedState>,
    Json(config): Json<NewJobConfig>,
) -> Result<AppResponse<JobConfig>, AppError> {
    info!("Creating config for job: {}-{}", config.application, config.job_name);

    let mut conn = state.pool.get().await?;

    let existing = get_job_config_by_application_and_name(
        &mut conn,
        &config.application,
        &config.job_name,
    ).await?;

    // 2. GUARD: If it exists, return an error (409 Conflict)
    if existing.is_some() {
        return Err(AppError::Conflict(format!(
            "Configuration already exists for application '{}' and job '{}'",
            config.application, config.job_name
        )));
    }

    let inserted = insert_config(&mut conn, config).await?;

    Ok(AppResponse::success_one("config", inserted))
}

pub async fn update_config_handler(
    State(state): State<SharedState>,
    Json(config): Json<JobConfig>,
) -> Result<AppResponse<JobConfig>, AppError> {
    info!("Updating config for job: {}-{}", config.application, config.job_name);

    let mut conn = state.pool.get().await?;

    let updated = save_config(&mut conn, config).await?;

    print!("{:?}", updated);

    Ok(AppResponse::success_one("config", updated))
}
