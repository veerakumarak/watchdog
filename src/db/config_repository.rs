use diesel::{QueryDsl, ExpressionMethods, OptionalExtension};
use diesel_async::RunQueryDsl;
use crate::db::connection::DbConnection;
use crate::errors::AppError;
use crate::models::{JobConfig, NewJobConfig};

pub async fn get_job_config_by_app_name_and_job_name(
    conn: &mut DbConnection<'_>,
    _app_name: &str,
    _job_name: &str,
) -> Result<Option<JobConfig>, AppError> {
    use crate::schema::job_configs::dsl::*;
    let job_config = job_configs
        .find((_app_name, _job_name))
        .first::<JobConfig>(conn)
        .await
        .optional()?;

    Ok(job_config)
}

pub async fn get_all_enabled_configs(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<JobConfig>, AppError> {
    let jobs = get_all_job_configs(conn).await?
        .into_iter()
        .filter(|job| job.enabled == false) // Safely check for true
        .collect();

    Ok(jobs)
}

pub async fn get_all_job_configs(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<JobConfig>, AppError> {
    use crate::schema::job_configs::dsl::*;
    let jobs = job_configs
        .load::<JobConfig>(conn)
        .await?;

    Ok(jobs)
}

pub async fn get_all_applications(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<String>, AppError> {
    use crate::schema::job_configs::dsl::*;

    let apps = job_configs
        .select(application)
        .distinct() // distinct ensures we don't get duplicates
        .load::<String>(conn)
        .await?;

    Ok(apps)
}

pub async fn get_jobs_by_application(
    conn: &mut DbConnection<'_>,
    _app_name: String,
) -> Result<Vec<JobConfig>, AppError> {
    use crate::schema::job_configs::dsl::*;

    let jobs = job_configs
        .filter(application.eq(_app_name))
        .load::<JobConfig>(conn)
        .await?;

    Ok(jobs)
}

pub async fn insert_config(
    conn: &mut DbConnection<'_>,
    new_config: NewJobConfig,
) -> Result<JobConfig, AppError> {
    let _app_name = &new_config.application;
    let _job_name = &new_config.job_name;

    use crate::schema::job_configs::dsl::*;
    let job_config = diesel::insert_into(job_configs)
        .values(&new_config)
        .get_result::<JobConfig>(conn)
        .await?;

    Ok(job_config)
}

pub async fn save_config(
    conn: &mut DbConnection<'_>,
    config: JobConfig,
) -> Result<JobConfig, AppError> {

    let target_app = config.application.clone();
    let target_job = config.job_name.clone();

    use crate::schema::job_configs::dsl::*;
    let job_config = diesel::update(job_configs.find((target_app, target_job)))
        .set(&config)
        .get_result::<JobConfig>(conn)
        .await?;

    Ok(job_config)
}
