use diesel::{QueryDsl, ExpressionMethods, OptionalExtension};
use diesel_async::RunQueryDsl;
use crate::db::connection::DbConnection;
use crate::errors::AppError;
use crate::models::{JobConfig, NewJobConfig};


pub async fn get_job_config_by_application_and_name(
    conn: &mut DbConnection<'_>,
    _application: &String,
    _job_name: &String,
) -> Result<Option<JobConfig>, AppError> {
    use crate::schema::job_configs::dsl::*;
    let job_config = job_configs
        .find((_application, _job_name))
        .first::<JobConfig>(conn)
        .await
        .optional()?;

    Ok(job_config)
}

pub async fn insert_config(
    conn: &mut DbConnection<'_>,
    new_config: NewJobConfig,
) -> Result<JobConfig, AppError> {

    let _app_name = &new_config.application;
    let _job_name = &new_config.job_name;

    // info!("Creating config for job: {:?}-{:?}", _app_name, _job_name);

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

    // info!("updating config for job: {}-{}", config.application, config.job_name);
    let target_app = config.application.clone();
    let target_job = config.job_name.clone();

    use crate::schema::job_configs::dsl::*;
    let job_config = diesel::update(job_configs.find((target_app, target_job)))
        .set(&config)
        .get_result::<JobConfig>(conn)
        .await?;

    Ok(job_config)
}

// pub fn update_config(
//     conn: &mut DbConnection<'_>,
//     app: &str,
//     job: &str,
//     changes: UpdateJobConfig,
// ) -> Result<JobConfig, AppError> {
//     use crate::schema::job_configs::dsl::*;
//
//     // 1. Define the target row using the composite primary key
//     let target = job_configs
//         .filter(application.eq(app))
//         .filter(job_name.eq(job));
//
//     // 2. Apply the changes from the UpdateJobConfig struct
//     diesel::update(target)
//         .set(changes)
//         .get_result(conn)
// }