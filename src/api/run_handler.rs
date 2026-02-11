use std::cmp::PartialEq;
use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use tracing::error;
use uuid::Uuid;
use crate::{SharedState};
use crate::core::job_run_matching::get_status;
use crate::core::job_stage_validations::check;
use crate::cron_utils::get_job_start_time;
use crate::db::config_repository::{get_job_config_by_app_name_and_job_name, save_config};
use crate::db::connection::DbConnection;
use crate::db::run_repository::{create_new_job_run, get_all_runs_top_100, get_job_run_by_id, get_latest_job_run_by_app_name_and_job_name, save_run};
use crate::dtos::job_run::JobRunDto;
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{JobConfig, JobRun, JobRunStage, JobRunStageStatus};
use crate::notification::core::{_handle_error, send_failed};
use crate::time_utils::{change_timezone, change_to_utc, get_utc_now};

pub async fn get_run_by_id_handler(
    State(state): State<SharedState>,
    Path(_run_id): Path<Uuid>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;

    let job_run_option = get_job_run_by_id(&mut conn, &_run_id).await?;

    if let Some(_job_run) = job_run_option {
        Ok(AppResponse::success_one("job-run", _job_run))
    } else {
        Err(AppError::NotFound(format!("Run instance doesn't exists for id '{}'", _run_id)))
    }
}

pub async fn get_all_runs_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<JobRunDto>>, AppError> {

    let mut conn = state.pool.get().await?;

    let job_runs = get_all_runs_top_100(&mut conn).await?;

    Ok(AppResponse::success_one("job-runs", job_runs.into_iter().map(Into::into).collect()))
}

pub async fn trigger_job_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
    // Optional: Trigger might need a body too (e.g., manual inputs)
    // Json(params): Json<TriggerParams>,
) -> Result<AppResponse<JobRun>, AppError> {

    let mut conn = state.pool.get().await?;

    let job_config_option = get_job_config_by_app_name_and_job_name(&mut conn, &app_name, &job_name).await?;

    if job_config_option.is_none() {
        return Err(AppError::NotFound(format!("resource not found for -{} {}", &app_name, &job_name)))
    }

    // let job_config = job_config_option.unwrap();

    let new_job_run = create_new_job_run(&mut conn, &app_name, &job_name).await?;

    Ok(AppResponse::success_one("job-run", new_job_run))
}

#[derive(Deserialize, Debug)] // Add Clone/Serialize if needed
pub enum JobRunStageEventType {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
#[derive(Deserialize, Debug)]
pub struct StageUpdatePayload {
    pub stage_name: String,
    pub event_type: JobRunStageEventType,
    pub message: Option<String>, // Optional: Good for error messages on failure
}
pub async fn update_stage_by_id_handler(
    State(state): State<SharedState>,
    Path(job_run_id): Path<String>,
    Json(payload): Json<StageUpdatePayload>,
) -> Result<AppResponse<JobRun>, AppError> {
    match payload.event_type {
        JobRunStageEventType::Started => {
            _job_run_start_handler(state, (None, Some(job_run_id.parse().unwrap()), payload.stage_name)).await
        },
        JobRunStageEventType::Completed => {
            _job_run_complete_handler(state, (None, Some(job_run_id.parse().unwrap()), payload.stage_name)).await
        },
        JobRunStageEventType::Failed => {
            _job_run_failed_handler(state, (None, Some(job_run_id.parse().unwrap()), payload.stage_name), payload.message).await
        },
    }
}

pub async fn update_stage_by_context_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
    Json(payload): Json<StageUpdatePayload>,
) -> Result<AppResponse<JobRun>, AppError> {
    match payload.event_type {
        JobRunStageEventType::Started => {
            _job_run_start_handler(state, (Some((app_name, job_name)), None, payload.stage_name)).await
        },
        JobRunStageEventType::Completed => {
            _job_run_complete_handler(state, (Some((app_name, job_name)), None, payload.stage_name)).await
        },
        JobRunStageEventType::Failed => {
            _job_run_failed_handler(state, (Some((app_name, job_name)), None, payload.stage_name), payload.message).await
        },
    }
}
async fn _job_run_start_handler(
    state: SharedState,
    (app_name_and_job_name_option, job_run_id_option, stage_name): (Option<(String, String)>, Option<Uuid>, String),
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let (_job_config, job_run) = job_run_update_stage(&state, &mut conn, app_name_and_job_name_option, job_run_id_option, &stage_name, JobRunStageType::Start, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}

async fn _job_run_complete_handler(
    state: SharedState,
    (app_name_and_job_name_option, job_run_id_option, stage_name): (Option<(String, String)>, Option<Uuid>, String),
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let (_job_config, job_run) = job_run_update_stage(&state, &mut conn, app_name_and_job_name_option, job_run_id_option, &stage_name, JobRunStageType::Complete, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}

async fn _job_run_failed_handler(
    state: SharedState,
    (app_name_and_job_name_option, job_run_id_option, stage_name): (Option<(String, String)>, Option<Uuid>, String),
    message: Option<String>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let (job_config, job_run) = job_run_update_stage(&state, &mut conn, app_name_and_job_name_option.clone(), job_run_id_option, &stage_name, JobRunStageType::Failed, JobRunStageStatus::Failed).await?;
    let res = send_failed(&state.dispatcher, &job_config, &job_run, &stage_name, &message.unwrap_or("".to_string()), &job_config.channel_ids).await;
    if let Err(err) = res {
        error!("failed to send failed notification: {:?} - {} - {} - {}", app_name_and_job_name_option, stage_name, job_run_id_option.map(|uuid| uuid.to_string()).unwrap_or_else(|| "None".to_string()), err.to_string());

        // 1. Get the data and drop the lock immediately
        let error_channels = {
            let _settings = state.settings.read().expect("Lock poisoned");
            _settings.error_channels.clone() // Clone the data out
        }; // Guard is dropped here

        // 2. Now await using the cloned data
        _handle_error(
            &state.dispatcher,
            app_name_and_job_name_option,
            job_run_id_option.map(|uuid| uuid.to_string()),
            &stage_name,
            &err.to_string(),
            &error_channels // Pass the clone
        ).await;
    }
    Ok(AppResponse::success_one("job-run", job_run))
}

#[derive(Clone, PartialEq)]
pub enum JobRunStageType {
    Start,
    Complete,
    Failed,
}

async fn job_run_update_stage(
    state: &SharedState,
    conn: &mut DbConnection<'_>,
    app_name_and_job_name_option: Option<(String, String)>,
    job_run_id_option: Option<Uuid>,
    stage_name: &String,
    stage_type: JobRunStageType,
    stage_status: JobRunStageStatus
) -> Result<(JobConfig, JobRun), AppError> {
    if app_name_and_job_name_option.is_none() && job_run_id_option.is_none() {
        return Err(AppError::BadRequest("Either (app_name and job_name) or job_run_id should be provided".to_string()))
    }

    let result;
    if let Some(job_run_id) = job_run_id_option {
        result = _job_run_update_stage_with_run_id(conn, job_run_id, stage_name, stage_type, stage_status).await;
    } else {
        let (app_name, job_name) = app_name_and_job_name_option.clone().unwrap();
        result = _job_run_update_stage_with_app_name_and_job_name(conn, app_name, job_name, stage_name, stage_type, stage_status).await;
    }

    /*
        if let Err(err) = result {
        error!("failed to update stage: {:?} - {} - {} - {}", app_name_and_job_name_option, stage_name, job_run_id_option.map(|uuid| uuid.to_string()).unwrap_or_else(|| "None".to_string()), err.to_string());
        {
            let _settings = state.settings.read().expect("Lock poisoned");
            _handle_error(&state.dispatcher, app_name_and_job_name_option, job_run_id_option.map(|uuid| uuid.to_string()), &stage_name, &err.to_string(), &_settings.error_channels).await;
        }
        Err(err)
    } else {
        result
    }

     */
    if let Err(err) = result {
        error!("failed to update stage: {:?} - {} - {} - {}", app_name_and_job_name_option, stage_name, job_run_id_option.map(|uuid| uuid.to_string()).unwrap_or_else(|| "None".to_string()), err.to_string());

        // 1. Get the data and drop the lock immediately
        let error_channels = {
            let _settings = state.settings.read().expect("Lock poisoned");
            _settings.error_channels.clone() // Clone the data out
        }; // Guard is dropped here

        // 2. Now await using the cloned data
        _handle_error(
            &state.dispatcher,
            app_name_and_job_name_option,
            job_run_id_option.map(|uuid| uuid.to_string()),
            &stage_name,
            &err.to_string(),
            &error_channels // Pass the clone
        ).await;
        Err(err)
    } else {
        result
    }
}
async fn _job_run_update_stage_with_run_id(
    conn: &mut DbConnection<'_>,
    job_run_id: Uuid,
    stage_name: &String,
    stage_type: JobRunStageType,
    stage_status: JobRunStageStatus
) -> Result<(JobConfig, JobRun), AppError> {
    let job_run = _get_job_run_by_id(conn, &job_run_id).await?;

    let (app_name, job_name) = (job_run.app_name.clone(), job_run.job_name.clone());

    let job_config = _get_job_config_by_app_name_and_job_name(conn, &app_name, &job_name).await?;

    let utc_now = get_utc_now();

    _job_run_update_stage_internal(conn, job_config, job_run, utc_now, stage_name, stage_type, stage_status).await
}

async fn _job_run_update_stage_with_app_name_and_job_name(
    conn: &mut DbConnection<'_>,
    app_name: String,
    job_name: String,
    stage_name: &String,
    stage_type: JobRunStageType,
    stage_status: JobRunStageStatus
) -> Result<(JobConfig, JobRun), AppError> {
    let job_config = _get_job_config_by_app_name_and_job_name(conn, &app_name, &job_name).await?;

    if job_config.zone_id.is_none() || job_config.schedule.is_none() {
        return Err(AppError::InternalError(format!("zone or schedule should not be empty {}-{}", &app_name, &job_name)))
    }
    let zone_id = job_config.zone_id.as_ref().unwrap();
    let utc_now = get_utc_now();
    let tz_now = change_timezone(&utc_now, zone_id)?;

    let job_start_time = get_job_start_time(&job_config, &tz_now)?;

    let job_run_option = get_latest_job_run_by_app_name_and_job_name(conn, &app_name, &job_name, &change_to_utc(&job_start_time)?).await?;

    let job_run = match job_run_option {
        Some(run) => run,
        None => create_new_job_run(
            conn,
            &job_config.app_name,
            &job_config.job_name
        ).await?,
    };

    _job_run_update_stage_internal(conn, job_config, job_run, utc_now, stage_name, stage_type, stage_status).await
}

async fn _job_run_update_stage_internal(
    conn: &mut DbConnection<'_>,
    mut job_config: JobConfig,
    mut job_run: JobRun,
    utc_now: DateTime<Utc>,
    stage_name: &String,
    stage_type: JobRunStageType,
    stage_status: JobRunStageStatus
) -> Result<(JobConfig, JobRun), AppError> {

    // Enable the job if paused
    if !job_config.enabled {
        job_config.enabled = true;
        job_config = save_config(conn, job_config).await?;
    }

    check(&stage_type, &job_config, stage_name)?;

    let mut new_stage = JobRunStage {
        name: stage_name.clone(),
        start_status: None,
        start_date_time: None,
        complete_status: None,
        complete_date_time: None,
    };
    if stage_type == JobRunStageType::Start {
        new_stage.start_status = Some(stage_status);
        new_stage.start_date_time = Some(utc_now);
    } else {
        new_stage.complete_status = Some(stage_status);
        new_stage.complete_date_time = Some(utc_now);
    }

    job_run.stages.push(new_stage);

    job_run.status = get_status(&job_config, &job_run);

    let updated = save_run(conn, job_run).await?;
    Ok((job_config, updated))
}

async fn _get_job_config_by_app_name_and_job_name(conn: &mut DbConnection<'_>, app_name: &String, job_name: &String) -> Result<JobConfig, AppError> {
    let job_config_option = get_job_config_by_app_name_and_job_name(conn, app_name, job_name).await?;
    if job_config_option.is_none() {
        return Err(AppError::NotFound(format!("job config not found for: {}-{}", app_name, job_name)))
    }
    Ok(job_config_option.unwrap())
}

async fn _get_job_run_by_id(conn: &mut DbConnection<'_>, job_run_id: &Uuid) -> Result<JobRun, AppError> {
    let job_run_option = get_job_run_by_id(conn, job_run_id).await?;
    if job_run_option.is_none() {
        return Err(AppError::NotFound(format!("job run not found for id: {}", job_run_id)))
    }
    Ok(job_run_option.unwrap())
}

