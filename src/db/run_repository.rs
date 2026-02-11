use chrono::{DateTime, Utc};
use diesel::{QueryDsl, OptionalExtension, ExpressionMethods};
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use crate::db::connection::DbConnection;
use crate::errors::AppError;
use crate::models::{JobRun, JobRunStatus, NewJobRun};
use crate::time_utils::get_utc_now;

pub async fn get_job_run_by_id(
    conn: &mut DbConnection<'_>,
    _run_id: &Uuid,
) -> Result<Option<JobRun>, AppError> {
    use crate::schema::job_runs::dsl::*;
    let job_run = job_runs
        .find(_run_id)
        .first::<JobRun>(conn)
        .await
        .optional()?;

    Ok(job_run)
}

pub async fn get_latest_job_run_by_app_name_and_job_name(
    conn: &mut DbConnection<'_>,
    _app_name: &str,
    _job_name: &str,
    _start_time: &DateTime<Utc>,
) -> Result<Option<JobRun>, AppError> {
    use crate::schema::job_runs::dsl::*;
    let job_run = job_runs
        .filter(app_name.eq(_app_name))
        .filter(job_name.eq(_job_name))
        .filter(triggered_at.ge(_start_time))
        .first::<JobRun>(conn)
        .await
        .optional()?;

    Ok(job_run)
}

pub async fn get_all_runs_top_100(conn: &mut DbConnection<'_>) -> Result<Vec<JobRun>, AppError> {
    use crate::schema::job_runs::dsl::*;
    let result = job_runs
        .limit(100)
        .load::<JobRun>(conn)
        .await?;

    Ok(result)
}

pub async fn get_all_pending_job_runs(
    conn: &mut DbConnection<'_>,
    time_boundary: DateTime<Utc>,
) -> Result<Vec<JobRun>, AppError> {
    use crate::schema::job_runs::dsl::*;
    let result = job_runs
        .filter(updated_at.ge(time_boundary))
        .load::<JobRun>(conn)
        .await?;

    Ok(result)
}

pub async fn create_new_job_run(
    conn: &mut DbConnection<'_>,
    _app_name: &String,
    _job_name: &String,
) -> Result<JobRun, AppError> {

    let new_job_run = NewJobRun {
        app_name: _app_name.clone(),
        job_name: _job_name.clone(),
        status: JobRunStatus::InProgress,
        stages: diesel_json::Json(Vec::new()),
        triggered_at: get_utc_now(),
    };

    insert_run(conn, new_job_run).await
}

pub async fn insert_run(
    conn: &mut DbConnection<'_>,
    new_run: NewJobRun,
) -> Result<JobRun, AppError> {
    use crate::schema::job_runs::dsl::*;
    let job_run = diesel::insert_into(job_runs)
        .values(&new_run)
        .get_result::<JobRun>(conn)
        .await?;

    Ok(job_run)
}

pub async fn save_run(
    conn: &mut DbConnection<'_>,
    run: JobRun,
) -> Result<JobRun, AppError> {
    use crate::schema::job_runs::dsl::*;
    let job_run = diesel::update(job_runs.find(&run.id))
        .set(&run)
        .get_result::<JobRun>(conn)
        .await?;

    Ok(job_run)
}
