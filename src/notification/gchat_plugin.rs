use crate::validations::validate_url;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;
use validator::Validate;
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun, ProviderType};
use crate::models::ProviderType::GchatWebhook;
use crate::notification::core::{AlertEvent, AlertType};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct GchatPlugin;

#[derive(Debug, Validate, Deserialize)]
struct Config {
    #[validate(custom(function = "validate_url"))]
    pub webhook_url: String,
}

#[async_trait]
impl NotificationPlugin for GchatPlugin {
    fn provider_type(&self) -> ProviderType {
        GchatWebhook
    }

    fn validate_config(&self, config: &Value) -> Result<(), AppError> {
        // match config.get("webhook_url").and_then(|v| v.as_str()) {
        //     Some(url) if url.starts_with("https://hooks.slack.com") => Ok(()),
        //     _ => Err(AppError::BadRequest(
        //         "Missing or invalid 'webhook_url'. Must start with slack hooks URL.".into(),
        //     )),
        // }

        let _config: Config = serde_json::from_value(config.clone()).map_err(|e| {
            AppError::BadRequest(format!("invalid config provided {}", e))
        })?;
        _config.validate()?;

        print!("{:?}", config);

        Ok(())
    }

    async fn send(&self, alert: &AlertEvent, config: &Value) -> Result<(), AppError> {
        let webhook_url = config["webhook_url"].as_str().unwrap(); // Safe due to validation
        info!(
            "[Gchat Plugin] Sending Gchat message to URL: {}\n\tMessage: [{}] {}",
            webhook_url, alert.severity, alert.message
        );

        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        Ok(())
    }

    async fn send2(&self, job_config: &JobConfig, job_run: &JobRun, stage_name: &String, config: &Value, alert_type: AlertType) -> Result<(), AppError> {
        let webhook_url = config["webhook_url"].as_str().unwrap(); // Safe due to validation
        let message = render_message(alert_type, job_config, job_run, stage_name);

        println!("sending to url: {}", webhook_url);

        let client = Client::new();

        let payload = json!({
            "text": message
        });
        println!("sending payload: {}", payload);

        let res = client.post(webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to build request: {}", e)))?;

        println!("{:?}", res);

        Ok(())
    }
}

fn render_message(alert_type: AlertType, job_config: &JobConfig, job_run: &JobRun, stage: &str) -> String {
    match alert_type {
        // AlertType::Error =>
        //     "🕵️ *Watchdog Error* 🕵️\n*Application*: {application}\n*Dag Name*: {dag}\n*Stage Name*: {stage}\n*Message*: {message}",
        AlertType::Timeout =>
            "⏳ Job Timeout ⏳\n*Application*: {app_name}\n*Job Name*: {job_name}\n*Stage Name*: {stage}\n*Run Id*: {run_id}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())
        ,
        AlertType::Failed =>
            "🚨 Job Failed 🚨\n*Application*: {app_name}\n*Job Name*: {job_name}\n*Stage Name*: {stage}\n*Run Id*: {run_id}\n*Message*: {message}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())
        ,
    }
}


