use std::time::Duration;
use tracing::info;
use crate::config::Config;
use crate::core::process_timeouts::check_all_timeouts;
use crate::db::connection::PgPool;
use crate::notification::dispatcher::NotificationDispatcher;

pub async fn scheduler(db: &PgPool, notification_dispatcher: &NotificationDispatcher, config: &Config) {

    info!("Starting scheduler. Waiting for initial delay...");

    // Initial Delay (2 seconds)
    tokio::time::sleep(Duration::from_millis(config.scheduler_initial_delay_ms)).await;
    info!("Initial delay complete. Starting scheduled task loop.");

    // Scheduled Task Loop
    loop {
        // Run the task
        check_all_timeouts(db, notification_dispatcher, config).await.expect("scheduler died");

        info!("Task completed. Waiting for fixed delay of {}ms...", config.scheduler_fixed_delay_ms);

        // Fixed Delay (30 seconds)
        tokio::time::sleep(Duration::from_millis(config.scheduler_fixed_delay_ms)).await;
    }
}
