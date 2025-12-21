use std::time::Duration;
use tracing::info;
use crate::config::Config;
use crate::core::process_timeouts::check_all_timeouts;
use crate::db::connection::PgPool;
use crate::notification::dispatcher::NotificationDispatcher;

pub async fn scheduler(db: &PgPool, notification_dispatcher: &NotificationDispatcher, config: &Config) {

    info!("Starting scheduler. Waiting for initial delay...");

    tokio::time::sleep(Duration::from_secs(config.scheduler_initial_delay_seconds)).await;

    info!("Initial delay complete. Starting scheduled task loop.");

    loop {
        check_all_timeouts(db, notification_dispatcher, config).await.expect("scheduler died");

        info!("Task completed. Waiting for fixed delay of {}secs...", config.scheduler_fixed_delay_seconds);

        tokio::time::sleep(Duration::from_secs(config.scheduler_fixed_delay_seconds)).await;
    }
}
