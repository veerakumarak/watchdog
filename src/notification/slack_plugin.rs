use async_trait::async_trait;
use serde_json::Value;
use crate::errors::AppError;
use crate::notification::core::{AlertEvent};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct SlackPlugin;

#[async_trait]
impl NotificationPlugin for SlackPlugin {
    fn provider_type(&self) -> &str {
        "slack_webhook"
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
        println!(
            "[Slack Plugin] Sending Slack message to URL: {}\n\tMessage: [{}] {}",
            webhook_url, alert.severity, alert.message
        );

        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        Ok(())
    }
}

