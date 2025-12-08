use crate::api::run_handler::JobRunStageType;
use crate::errors::AppError;
use crate::models::{JobConfig, JobStageConfig};

fn check_and_return_job_stage<'a>(
    job_config: &'a JobConfig,
    stage_name: &str,
) -> Result<&'a JobStageConfig, AppError> {
    job_config.stages
        .iter()
        .find(|config| config.name == stage_name)
        .ok_or_else(|| {
            AppError::BadRequest(format!("invalid stage name provided {}", stage_name))
        })
}
pub fn check(
    event_stage_type: &JobRunStageType,
    job_config: &JobConfig,
    stage_name: &str,
) -> Result<(), AppError> {
    match event_stage_type {
        JobRunStageType::Start => check_valid_start(job_config, stage_name),
        JobRunStageType::Complete => check_valid_complete(job_config, stage_name),
        _ => Ok(()),
    }
}

pub fn check_valid_start(job_config: &JobConfig, stage_name: &str) -> Result<(), AppError> {
    let job_stage = check_and_return_job_stage(job_config, stage_name)?;

    if job_stage.start.is_none() {
        return Err(AppError::BadRequest(format!("start not configured for the stage {}", stage_name)));
    }

    Ok(())
}

pub fn check_valid_complete(job_config: &JobConfig, stage_name: &str) -> Result<(), AppError> {
    let job_stage = check_and_return_job_stage(job_config, stage_name)?;

    if job_stage.complete.is_none() {
        return Err(AppError::BadRequest(format!("complete not configured for the stage {}", stage_name)));
    }

    Ok(())
}
