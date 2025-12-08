use axum::extract::{Path, State};
use axum::Json;
use tracing::info;
use crate::{SharedState};
use crate::db::config_repository::{get_all_applications, get_all_job_configs, get_job_config_by_app_name_and_job_name, get_jobs_by_application, insert_config, save_config};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{JobConfig, NewJobConfig};

pub async fn get_config_by_app_name_and_job_name_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
) -> Result<AppResponse<JobConfig>, AppError> {
    let mut conn = state.pool.get().await?;

    let job_config = get_job_config_by_app_name_and_job_name(&mut conn, &app_name, &job_name).await?;

    if let Some(_job_config) = job_config {
        Ok(AppResponse::success_one("config", _job_config))
    } else {
        Err(AppError::NotFound(format!("Configuration doesn't exists for application '{}' and job '{}'", app_name, job_name)))
    }
}

pub async fn create_config_handler(
    State(state): State<SharedState>,
    Json(job_config): Json<NewJobConfig>,
) -> Result<AppResponse<JobConfig>, AppError> {
    info!("Creating config for job: {}-{}", job_config.application, job_config.job_name);

    let mut conn = state.pool.get().await?;

    let existing = get_job_config_by_app_name_and_job_name(
        &mut conn,
        &job_config.application,
        &job_config.job_name,
    ).await?;

    // 2. GUARD: If it exists, return an error (409 Conflict)
    if existing.is_some() {
        return Err(AppError::Conflict(format!(
            "Configuration already exists for application '{}' and job '{}'",
            job_config.application, job_config.job_name
        )));
    }

    let inserted = insert_config(&mut conn, job_config).await?;

    Ok(AppResponse::success_one("job-config", inserted))
}

pub async fn update_config_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
    Json(job_config): Json<JobConfig>,
) -> Result<AppResponse<JobConfig>, AppError> {
    info!("Updating config for job: {}-{}", job_config.application, job_config.job_name);

    // ToDo - change application to app_name
    if app_name != job_config.application {
        return Err(AppError::BadRequest("invalid app_name provided".into()));
    }
    if job_name != job_config.job_name {
        return Err(AppError::BadRequest("invalid job_name provided".into()));
    }

    let mut conn = state.pool.get().await?;

    // ToDo - update update code
    let updated = save_config(&mut conn, job_config).await?;

    Ok(AppResponse::success_one("job-config", updated))
}

pub async fn get_all_configs_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<JobConfig>>, AppError> {

    let mut conn = state.pool.get().await?;

    let jobs = get_all_job_configs(&mut conn).await?;

    Ok(AppResponse::success_one("job-configs", jobs))
}

pub async fn get_all_applications_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<String>>, AppError> {

    let mut conn = state.pool.get().await?;

    let applications = get_all_applications(&mut conn).await?;

    Ok(AppResponse::success_one("applications", applications))
}

pub async fn list_jobs_by_app_handler(
    State(state): State<SharedState>,
    Path(app_name): Path<String>, // Extract app_name from URL
) -> Result<AppResponse<Vec<JobConfig>>, AppError> {

    let mut conn = state.pool.get().await?;

    let jobs = get_jobs_by_application(&mut conn, app_name).await?;

    Ok(AppResponse::success_one("job-configs", jobs))
}