use std::time::Duration;
use tracing::info;
use crate::core::process_timeouts::check_all_timeouts;
use crate::db::connection::PgPool;
use crate::notification::dispatcher::NotificationDispatcher;
use crate::{SharedState};

pub async fn scheduler(db: &PgPool, notification_dispatcher: &NotificationDispatcher, state: SharedState) {

    info!("Starting scheduler. Waiting for initial delay...");

    let config = state.config.clone();

    tokio::time::sleep(Duration::from_secs(config.scheduler_initial_delay_seconds)).await;

    info!("Initial delay complete. Starting scheduled task loop.");


    loop {
        let current_settings = {
            let _settings = state.settings.read().expect("Lock poisoned");
            _settings.clone()
        };

        check_all_timeouts(db, notification_dispatcher, &config, current_settings.clone()).await.expect("scheduler died");

        info!("Task completed. Waiting for fixed delay of {}secs...", config.scheduler_fixed_delay_seconds);

        tokio::time::sleep(Duration::from_secs(config.scheduler_fixed_delay_seconds)).await;
    }
}
