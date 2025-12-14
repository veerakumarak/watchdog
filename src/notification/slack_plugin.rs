use async_trait::async_trait;
use serde_json::Value;
use tracing::info;
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun, ProviderType};
use crate::models::ProviderType::SlackWebhook;
use crate::notification::core::{AlertEvent, AlertType};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct SlackPlugin;

#[async_trait]
impl NotificationPlugin for SlackPlugin {
    fn provider_type(&self) -> ProviderType {
        SlackWebhook
    }

    fn validate_config(&self, config: &Value) -> Result<(), AppError> {
        // Ensure the config has a "webhook_url" string field.
        match config.get("webhook_url").and_then(|v| v.as_str()) {
            Some(url) if url.starts_with("https://hooks.slack.com") => Ok(()),
            _ => Err(AppError::BadRequest(
                "Missing or invalid 'webhook_url'. Must start with slack hooks URL.".into(),
            )),
        }
    }

    async fn send(&self, alert: &AlertEvent, config: &Value) -> Result<(), AppError> {
        let webhook_url = config["webhook_url"].as_str().unwrap(); // Safe due to validation
        info!(
            "[Slack Plugin] Sending Slack message to URL: {}\n\tMessage: [{}] {}",
            webhook_url, alert.severity, alert.message
        );

        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        Ok(())
    }

    async fn send2(&self, job_config: &JobConfig, job_run: &JobRun, config: &Value, alert_type: AlertType) -> Result<(), AppError> {
        todo!()
    }

}

fn render_message(alert_type: AlertType, job_config: &JobConfig, job_run: &JobRun, stage: &str) -> String {
    match alert_type {
        // AlertType::Error =>
        //     "🕵️ *Watchdog Error* 🕵️\n*Application*: {application}\n*Dag Name*: {dag}\n*Stage Name*: {stage}\n*Message*: {message}",
        AlertType::Timeout =>
            "⏳ Job Timeout ⏳\n*Application*: {application}\n*Dag Name*: {dag}\n*Stage Name*: {stage}\n*Run Id*: {run_id}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())
        ,
        AlertType::Failed =>
            "🚨 Job Failed 🚨\n*Application*: {application}\n*Dag Name*: {dag}\n*Stage Name*: {stage}\n*Run Id*: {run_id}\n*Message*: {message}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())
        ,
    }
}

