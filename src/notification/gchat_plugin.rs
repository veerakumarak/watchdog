use crate::validations::validate_url;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;
use crate::errors::AppError;
use crate::models::{ProviderType};
use crate::models::ProviderType::GchatWebhook;
use crate::notification::core::{AlertType};
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

    async fn send(&self, app_name: &String, job_name: &String, run_id_opt: Option<String>, stage_name: &String, message_opt: Option<String>, config: &Value, alert_type: AlertType) -> Result<(), AppError> {
        let webhook_url = config["webhook_url"].as_str().unwrap(); // Safe due to validation
        let message = render_message(alert_type, app_name, job_name, run_id_opt, stage_name, message_opt);

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

fn render_message(alert_type: AlertType, app_name: &String, job_name: &String, run_id_opt: Option<String>, stage: &str, message_opt: Option<String>) -> String {
    match alert_type {
        AlertType::Error =>
            "🕵️ *Watchdog Error* 🕵️\n*Application*: {app_name}\n*Job Name*: {job_name}\n*Stage Name*: {stage}\n*Run Id*: {run_id}\n*Message*: {message}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
                .replace("{message}", &message_opt.unwrap_or("".to_string()))
        ,
        AlertType::Timeout =>
            "⏳ Job Timeout ⏳\n*Application*: {app_name}\n*Job Name*: {job_name}\n*Stage Name*: {stage}\n*Run Id*: {run_id}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
        ,
        AlertType::Failed =>
            "🚨 Job Failed 🚨\n*Application*: {app_name}\n*Job Name*: {job_name}\n*Stage Name*: {stage}\n*Run Id*: {run_id}\n*Message*: {message}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
                .replace("{message}", &message_opt.unwrap_or("".to_string()))
        ,
    }
}


