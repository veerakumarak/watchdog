use std::collections::HashMap;
use std::sync::Arc;
use crate::db::connection::PgPool;
use crate::models::ProviderType::{EmailSmtp, GchatWebhook};
use crate::notification::dispatcher::NotificationDispatcher;
use crate::notification::email_plugin::EmailPlugin;
use crate::notification::gchat_plugin::GchatPlugin;
use crate::notification::plugin_registry::PluginRegistry;

pub async fn init_notification(db: PgPool) -> NotificationDispatcher {

    let mut registry: PluginRegistry = HashMap::new();
    registry.insert(GchatWebhook, Arc::new(GchatPlugin));
    registry.insert(EmailSmtp, Arc::new(EmailPlugin));

    NotificationDispatcher::new(db, registry)
}
