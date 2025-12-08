use async_trait::async_trait;
use serde_json::Value;
use crate::errors::AppError;
use crate::notification::core::{AlertEvent};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct EmailPlugin;

#[async_trait]
impl NotificationPlugin for EmailPlugin {
    fn provider_type(&self) -> &str {
        "smtp_email"
    }

    fn validate_config(&self, config: &Value) -> Result<(), AppError> {
        if config.get("smtp_host").is_some()
            && config.get("smtp_port").is_some()
            && config.get("to_addresses").is_some_and(|v| v.is_array())
        {
            Ok(())
        } else {
            Err(AppError::BadRequest(
                "Missing smtp_host, smtp_port, or to_addresses array".into(),
            ))
        }
    }

    async fn send(&self, alert: &AlertEvent, config: &Value) -> Result<(), AppError> {
        let host = config["smtp_host"].as_str().unwrap_or("localhost");
        let to = config["to_addresses"].as_array().unwrap();
        println!(
            "[Email Plugin] Connecting to SMTP server at {}. Sending email to {:?}.\n\tSubject: Alert {}\n\tBody: {}",
            host, to, alert.id, alert.message
        );

        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        Ok(())
    }
}
