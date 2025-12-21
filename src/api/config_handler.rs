use axum::extract::{Path, State};
use axum::Json;
use tracing::info;
use validator::Validate;
use crate::{SharedState};
use crate::db::config_repository::{get_all_applications, get_all_job_configs, get_job_config_by_app_name_and_job_name, get_jobs_by_application, insert_config, save_config};
use crate::dtos::job_config::{JobConfigCreateRequest, JobConfigDto, JobConfigUpdateRequest};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{NewJobConfig};

pub async fn get_config_by_app_name_and_job_name_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
) -> Result<AppResponse<JobConfigDto>, AppError> {
    let mut conn = state.pool.get().await?;

    let job_config = get_job_config_by_app_name_and_job_name(&mut conn, &app_name, &job_name).await?;

    if let Some(_job_config) = job_config {
        Ok(AppResponse::success_one("config", _job_config.into()))
    } else {
        Err(AppError::NotFound(format!("Configuration doesn't exists for application '{}' and job '{}'", app_name, job_name)))
    }
}

pub async fn create_config_handler(
    State(state): State<SharedState>,
    Json(_create_request): Json<JobConfigCreateRequest>,
) -> Result<AppResponse<JobConfigDto>, AppError> {
    _create_request.validate()?;
    info!("Creating config for job: {}-{}", _create_request.app_name, _create_request.job_name);

    let mut conn = state.pool.get().await?;

    let existing = get_job_config_by_app_name_and_job_name(
        &mut conn,
        &_create_request.app_name,
        &_create_request.job_name,
    ).await?;

    // 2. GUARD: If it exists, return an error (409 Conflict)
    if existing.is_some() {
        return Err(AppError::Conflict(format!(
            "Configuration already exists for application '{}' and job '{}'",
            _create_request.app_name, _create_request.job_name
        )));
    }

    let _new_job_config = NewJobConfig {
        app_name: _create_request.app_name,
        job_name: _create_request.job_name,
        schedule: _create_request.schedule,
        zone_id: _create_request.zone_id,
        stages: _create_request.stages,
        channel_ids: _create_request.channel_ids,
    };
    
    let inserted = insert_config(&mut conn, _new_job_config).await?;
    Ok(AppResponse::success_one("job-config", inserted.into()))
}

pub async fn update_config_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
    Json(_update_request): Json<JobConfigUpdateRequest>,
) -> Result<AppResponse<JobConfigDto>, AppError> {
    _update_request.validate()?;

    info!("Updating config for job: {}-{}", app_name, job_name);

    let mut conn = state.pool.get().await?;

    let mut _job_config = get_job_config_by_app_name_and_job_name(&mut conn, &app_name, &job_name)
        .await?
        .ok_or(AppError::NotFound(format!("JobConfig doesn't exists for app_name '{}' and job name '{}'", app_name, job_name)))?;

    _job_config.schedule = _update_request.schedule;
    _job_config.zone_id = _update_request.zone_id;
    _job_config.stages = _update_request.stages;
    _job_config.channel_ids = _update_request.channel_ids;
    
    let updated = save_config(&mut conn, _job_config).await?;
    Ok(AppResponse::success_one("job-config", updated.into()))
}

pub async fn get_all_configs_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<JobConfigDto>>, AppError> {

    let mut conn = state.pool.get().await?;

    let jobs = get_all_job_configs(&mut conn).await?;

    Ok(AppResponse::success_one("job-configs", jobs.into_iter().map(Into::into).collect()))
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
) -> Result<AppResponse<Vec<JobConfigDto>>, AppError> {

    let mut conn = state.pool.get().await?;

    let jobs = get_jobs_by_application(&mut conn, app_name).await?;

    Ok(AppResponse::success_one("job-configs", jobs.into_iter().map(Into::into).collect()))
}