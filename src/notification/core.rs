use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun};
use crate::notification::core::AlertType::{Error, Failed, Timeout};
use crate::notification::dispatcher::NotificationDispatcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    Error,
    Timeout,
    Failed,
}

pub async fn send_timeout(dispatcher: &NotificationDispatcher, job_config: &JobConfig, job_run: &JobRun, stage_name: &str) -> Result<(), AppError>  {
    dispatcher.dispatch(&job_config.app_name, &job_config.job_name, Some(job_run.id.to_string()), stage_name, &job_config.channel_ids, None, Timeout).await
}

pub async fn send_failed(dispatcher: &NotificationDispatcher, job_config: &JobConfig, job_run: &JobRun, stage_name: &str, message: &String, channel_ids_str: &str) -> Result<(), AppError> {
    dispatcher.dispatch(&job_config.app_name, &job_config.job_name, Some(job_run.id.to_string()), stage_name, channel_ids_str, Some(message.to_string()), Failed).await
}

pub async fn send_error(dispatcher: &NotificationDispatcher, app_name: &String, job_name: &String, job_run_id_opt: Option<String>, stage_name: &str, message: &String, channel_ids_str: &str)  -> Result<(), AppError> {
    dispatcher.dispatch(app_name, job_name, job_run_id_opt, stage_name, channel_ids_str, Some(message.to_string()), Error).await
}
