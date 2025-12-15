use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;
use crate::errors::AppError;
use crate::models::{ProviderType};
use crate::notification::core::{AlertType};

#[async_trait]
pub trait NotificationPlugin: Send + Sync {
    /// Returns the unique string identifier for this plugin type (e.g., "slack", "email").
    fn provider_type(&self) -> ProviderType;

    /// Validates arbitrary JSON configuration before saving it to the DB.
    fn validate_config(&self, config: &Value) -> Result<(), AppError>;

    /// The core logic to execute the notification.
    /// It takes the generic alert and the provider-specific JSON config.
    async fn send(&self, app_name: &String, job_name: &String, run_id_opt: Option<String>, stage_name: &String, message_opt: Option<String>, config: &Value, alert_type: AlertType) -> Result<(), AppError>;
}

pub(crate) type PluginRegistry = HashMap<ProviderType, Arc<dyn NotificationPlugin>>;

