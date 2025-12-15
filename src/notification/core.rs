use tracing::info;
use serde::{Deserialize, Serialize};
use crate::models::{JobConfig, JobRun};
use crate::notification::core::AlertType::{Failed, Timeout};
use crate::notification::dispatcher::NotificationDispatcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub message: String,
    pub severity: String,
}

pub async fn send_error(dispatcher: &NotificationDispatcher, app_name: &str, job_name: &str, message: &str, channels: Vec<String>) {
    info!("in send error");
    let alert_info = AlertEvent {
        id: uuid::Uuid::new_v4().to_string(),
        message: message.to_string(),
        severity: "ERROR".to_string(),
    };
    dispatcher.dispatch(alert_info, channels).await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    // Error,
    Timeout,
    Failed,
}

pub async fn send_failed(dispatcher: &NotificationDispatcher, job_config: &JobConfig, job_run: &JobRun, stage_name: &str, message: &String, channel_ids_str: &str) {
    dispatcher.dispatch2(job_config, job_run, stage_name, channel_ids_str, Some(message.to_string()), Failed).await;
}
pub async fn send_timeout(dispatcher: &NotificationDispatcher, job_config: &JobConfig, job_run: &JobRun, stage_name: &str, channel_ids_str: &str) {
    dispatcher.dispatch2(job_config, job_run, stage_name, channel_ids_str, None, Timeout).await;
}
