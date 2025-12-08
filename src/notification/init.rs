use std::collections::HashMap;
use std::sync::Arc;
use crate::db::connection::PgPool;
use crate::notification::dispatcher::NotificationDispatcher;
use crate::notification::email_plugin::EmailPlugin;
use crate::notification::plugin_registry::PluginRegistry;
use crate::notification::slack_plugin::SlackPlugin;

pub async fn init_notification(db: PgPool) -> NotificationDispatcher {

    let mut registry: PluginRegistry = HashMap::new();
    registry.insert("slack_webhook".to_string(), Arc::new(SlackPlugin));
    registry.insert("smtp_email".to_string(), Arc::new(EmailPlugin));

    
    NotificationDispatcher::new(db, registry)
}
