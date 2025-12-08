use serde::{Deserialize, Serialize};
use crate::models::JobRun;
use crate::notification::dispatcher::NotificationDispatcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub message: String,
    pub severity: String,
}

pub async fn send_failed(dispatcher: &NotificationDispatcher, application: &str, job_name: &str, run: &JobRun, stage_name: &str, message: &str, channels: Vec<String>) {
    let alert_info = AlertEvent {
        id: run.id.to_string(),
        message: message.to_string(),
        severity: "CRITICAL".to_string(),
    };
    dispatcher.dispatch(alert_info, channels).await;
}
pub async fn send_timeout(dispatcher: &NotificationDispatcher, application: &str, job_name: &str, run: &JobRun, stage_name: &str, message: &str, channels: Vec<String>) {
    let alert_info = AlertEvent {
        id: run.id.to_string(),
        message: message.to_string(),
        severity: "CRITICAL".to_string(),
    };
    dispatcher.dispatch(alert_info, channels).await;
}
pub async fn send_error(dispatcher: &NotificationDispatcher, application: &str, job_name: &str, message: &str, channels: Vec<String>) {
    let alert_info = AlertEvent {
        id: uuid::Uuid::new_v4().to_string(),
        message: message.to_string(),
        severity: "ERROR".to_string(),
    };
    dispatcher.dispatch(alert_info, channels).await;
}
