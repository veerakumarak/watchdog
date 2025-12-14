use serde_json::Value;
use tracing::{error, info};
use crate::db::channel_repository::get_channel_by_id;
use crate::db::connection::PgPool;
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun, ProviderType};
use crate::notification::core::{AlertEvent, AlertType};
use crate::notification::plugin_registry::PluginRegistry;

#[derive(Clone)]
pub struct NotificationDispatcher {
    db: PgPool,
    registry: PluginRegistry,
}

impl NotificationDispatcher {
    pub fn new(
        db: PgPool,
        registry: PluginRegistry,
    ) -> Self {
        Self {
            db,
            registry,
        }
    }

    pub async fn validate(&self, _provider_type: &ProviderType, config: &Value) -> Result<(), AppError> {
        if let Some(plugin) = self.registry.get(_provider_type) {
            plugin.validate_config(config)
        } else {
            Err(AppError::BadRequest(format!("No plugin registered for type '{}' found in channel config.", _provider_type)))   
        }
    }

    pub async fn dispatch2(&self, job_config: &JobConfig, job_run: &JobRun, stage_name: &str, channel_ids_str: &str, alert_type: AlertType) {
        let mut join_handles = vec![];

        let mut conn = self.db.get().await.unwrap();

        let channel_ids: Vec<String> = channel_ids_str.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        for channel_id in channel_ids {
            // 1. Simulate fetching channel config from DB based on ID
            if let Some(channel_cfg) = get_channel_by_id(&mut conn, &channel_id).await.unwrap() {
                // 2. Look up the plugin implementation in the registry based on type string
                if let Some(plugin) = self.registry.get(&channel_cfg.provider_type) {
                    // Prepare data for async move
                    let config_clone = channel_cfg.configuration.clone();
                    // Clone the Arc pointer to the plugin implementation
                    let plugin_ref = plugin.clone();
                    let channel_name = channel_cfg.name.clone();
                    let job_config_clone = job_config.clone();
                    let job_run_clone = job_run.clone();
                    let stage_name_clone = stage_name.to_string();
                    let alert_type_clone = alert_type.clone();

                    // 3. Spawn an async task for execution so channels don't block each other.
                    let handle = tokio::spawn(async move {
                        info!("-> Sending via channel2: '{}'", channel_name);
                        match plugin_ref.send2(&job_config_clone, &job_run_clone, &config_clone, alert_type_clone).await {
                            Ok(_) => info!("<- Successfully sent via '{}'", channel_name),
                            Err(e) => error!("<- Failed to send via '{}': {}", channel_name, e),
                        }
                    });
                    join_handles.push(handle);

                } else {
                    error!(
                        "Error: No plugin registered for type '{}' found in channel config '{}'",
                        channel_cfg.provider_type, channel_id
                    );
                }
            } else {
                error!("Error: Channel ID '{}' not found in database.", channel_id);
            }
        }

        // Wait for all notifications to finish (optional, depending on requirements)
        for handle in join_handles {
            let _ = handle.await;
        }
        info!("--- Dispatch Complete ---");
    }

        /// The main entry point. The alerting system calls this.
    /// `channel_ids` are the DB IDs of the channels configured for this specific alert rule.
    pub async fn dispatch(&self, alert: AlertEvent, channel_ids: Vec<String>) {
        info!("--- Dispatching Alert {} ---", alert.id);

        let mut join_handles = vec![];

        let mut conn = self.db.get().await.unwrap();

        for channel_id in channel_ids {
            // 1. Simulate fetching channel config from DB based on ID
            if let Some(channel_cfg) = get_channel_by_id(&mut conn, &channel_id).await.unwrap() {
                // 2. Look up the plugin implementation in the registry based on type string
                if let Some(plugin) = self.registry.get(&channel_cfg.provider_type) {
                    // Prepare data for async move
                    let alert_clone = alert.clone();
                    let config_clone = channel_cfg.configuration.clone();
                    // Clone the Arc pointer to the plugin implementation
                    let plugin_ref = plugin.clone();
                    let channel_name = channel_cfg.name.clone();

                    // 3. Spawn an async task for execution so channels don't block each other.
                    let handle = tokio::spawn(async move {
                        info!("-> Sending via channel: '{}'", channel_name);
                        match plugin_ref.send(&alert_clone, &config_clone).await {
                            Ok(_) => info!("<- Successfully sent via '{}'", channel_name),
                            Err(e) => error!("<- Failed to send via '{}': {}", channel_name, e),
                        }
                    });
                    join_handles.push(handle);

                } else {
                    error!(
                        "Error: No plugin registered for type '{}' found in channel config '{}'",
                        channel_cfg.provider_type, channel_id
                    );
                }
            } else {
                error!("Error: Channel ID '{}' not found in database.", channel_id);
            }
        }

        // Wait for all notifications to finish (optional, depending on requirements)
        for handle in join_handles {
            let _ = handle.await;
        }
        info!("--- Dispatch Complete ---");
    }
}

