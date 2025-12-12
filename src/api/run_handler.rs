use std::cmp::PartialEq;
use axum::extract::{Path, State};
use uuid::Uuid;
use crate::{SharedState};
use crate::core::job_run_matching::get_status;
use crate::core::job_stage_validations::check;
use crate::cron_utils::get_job_start_time;
use crate::db::config_repository::{get_job_config_by_app_name_and_job_name, save_config};
use crate::db::connection::DbConnection;
use crate::db::run_repository::{create_new_job_run, get_job_run_by_id, get_latest_job_run_by_app_name_and_job_name, insert_run, save_run};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{JobRun, JobRunStage, JobRunStageStatus, JobRunStatus, NewJobRun};
use crate::notification::core::send_failed;
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

pub async fn job_run_trigger_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name)): Path<(String, String)>,
) -> Result<AppResponse<JobRun>, AppError> {

    let mut conn = state.pool.get().await?;

    let job_config_option = get_job_config_by_app_name_and_job_name(&mut conn, &app_name, &job_name).await?;

    if job_config_option.is_none() {
        return Err(AppError::NotFound(format!("resource not found for -{} {}", &app_name, &job_name)))
    }

    // let job_config = job_config_option.unwrap();

    let new_job_run = NewJobRun {
        application: app_name,
        job_name,
        triggered_at: get_utc_now(),
        status: JobRunStatus::InProgress,
        stages: diesel_json::Json(Vec::new()),
    };

    let inserted = insert_run(&mut conn, new_job_run).await?;
    Ok(AppResponse::success_one("job-run", inserted))
}

pub async fn job_run_start_with_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, job_run_id, stage_name)): Path<(String, String, Uuid, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let job_run = job_run_update_stage(&mut conn, &app_name, &job_name, Some(job_run_id), &stage_name, JobRunStageType::Start, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}
pub async fn job_run_start_without_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, stage_name)): Path<(String, String, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let job_run = job_run_update_stage(&mut conn, &app_name, &job_name, None, &stage_name, JobRunStageType::Start, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}

pub async fn job_run_complete_with_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, job_run_id, stage_name)): Path<(String, String, Uuid, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let job_run = job_run_update_stage(&mut conn, &app_name, &job_name, Some(job_run_id), &stage_name, JobRunStageType::Complete, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}
pub async fn job_run_complete_without_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, stage_name)): Path<(String, String, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let job_run = job_run_update_stage(&mut conn, &app_name, &job_name, None, &stage_name, JobRunStageType::Complete, JobRunStageStatus::Occurred).await?;
    Ok(AppResponse::success_one("job-run", job_run))
}

pub async fn job_run_failed_with_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, job_run_id, stage_name)): Path<(String, String, Uuid, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let result = job_run_update_stage(&mut conn, &app_name, &job_name, Some(job_run_id), &stage_name, JobRunStageType::Failed, JobRunStageStatus::Failed).await;
    if let Ok(job_run) = result {
        send_failed(&state.dispatcher, &app_name, &job_name, &job_run, &stage_name, "Job failed", vec!["slack_webhook".to_string()]).await;
        Ok(AppResponse::success_one("job-run", job_run))
    } else {
        Err(result.err().unwrap())
    }
}
pub async fn job_run_failed_without_run_id_handler(
    State(state): State<SharedState>,
    Path((app_name, job_name, stage_name)): Path<(String, String, String)>,
) -> Result<AppResponse<JobRun>, AppError> {
    let mut conn = state.pool.get().await?;
    let result = job_run_update_stage(&mut conn, &app_name, &job_name, None, &stage_name, JobRunStageType::Failed, JobRunStageStatus::Failed).await;
    if let Ok(job_run) = result {
        send_failed(&state.dispatcher, &app_name, &job_name, &job_run, &stage_name, "Job failed", vec!["slack_webhook".to_string()]).await;
        Ok(AppResponse::success_one("job-run", job_run))
    } else {
        Err(result.err().unwrap())
    }
}

#[derive(PartialEq)]
pub enum JobRunStageType {
    Start,
    Complete,
    Failed,
}

async fn job_run_update_stage(
    conn: &mut DbConnection<'_>,
    app_name: &str,
    job_name: &str,
    job_run_id_option: Option<Uuid>,
    stage_name: &String,
    stage_type: JobRunStageType,
    stage_status: JobRunStageStatus
) -> Result<JobRun, AppError> {

    let job_config_option = get_job_config_by_app_name_and_job_name(conn, &app_name, &job_name).await?;
    if job_config_option.is_none() {
        return Err(AppError::NotFound(format!("job config not found for: {}-{}", &app_name, &job_name)))
    }
    let mut job_config = job_config_option.unwrap();

    let utc_now = get_utc_now();

    let mut job_run;
    if let Some(job_run_id) = job_run_id_option {
        let job_run_option = get_job_run_by_id(conn, &job_run_id).await?;
        if job_run_option.is_none() {
            return Err(AppError::NotFound(format!("job run not found for id: {}", &job_run_id)))
        }
        job_run = job_run_option.unwrap();
    } else {
        if job_config.zone_id.is_none() || job_config.schedule.is_none() {
            return Err(AppError::InternalError(format!("zone or schedule should not be empty {}-{}", &app_name, &job_name)))
        }
        let zone_id = job_config.zone_id.as_ref().unwrap();
        let tz_now = change_timezone(&utc_now, zone_id)?;

        let job_start_time = get_job_start_time(&job_config, &tz_now)?;

        let job_run_option = get_latest_job_run_by_app_name_and_job_name(conn, app_name, job_name, &change_to_utc(&job_start_time)?).await?;

        if job_run_option.is_some() {
            job_run = job_run_option.unwrap();
        } else {
            job_run = create_new_job_run(conn, job_config.app_name.clone(), job_config.job_name.clone()).await?;
        }
    }

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
    Ok(updated)
}
