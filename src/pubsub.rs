use diesel_async::RunQueryDsl;
use futures::StreamExt;
use crate::db::connection::PgPool;
use crate::models::Settings;
use crate::SharedState;

pub async fn start_listener(pool: PgPool, state: SharedState) {
    tokio::spawn(async move {
        loop {
            // We handle the connection result manually because tokio::spawn
            // usually expects a closure that returns ()
            let mut connection = match pool.get().await {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("❌ Failed to get connection from pool: {}", e);
                    // eprintln!("📡 Listener error: {}. Retrying in 5s...", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            // Now you can proceed with the listener logic
            if let Err(e) = diesel::sql_query("LISTEN settings_update")
                .execute(&mut connection)
                .await
            {
                eprintln!("❌ Failed to LISTEN: {}", e);
                return;
            }

            let stream = connection.notifications_stream();
            tokio::pin!(stream);

            while let Some(notification_result) = stream.next().await {
                match notification_result {
                    Ok(notification) => {
                        // DESERIALIZE the JSON payload
                        let raw_payload = &notification.payload;
                        match serde_json::from_str::<Settings>(&*raw_payload) {
                            Ok(data) => {
                                println!("🔔 Received {:?} for id: {}", data, data.id);
                                {
                                    let mut config_lock = state.settings.write().expect("Failed to acquire write lock");
                                    *config_lock = data;
                                }
                                // Refined Logic: Only update specific cache key or re-fetch row by ID
                                // refresh_specific_setting(data.id, pool.clone(), current_settings.clone()).await;
                            }
                            Err(e) => eprintln!("❌ Failed to parse JSON payload: {}", e),
                        }
                        println!(
                            "🔔 Settings changed! Channel: {}, Payload: {}",
                            notification.channel, notification.payload
                        );
                        // Your refresh logic goes here
                    }
                    Err(e) => {
                        eprintln!("❌ Notification stream error: {}", e);
                        break;
                    }
                }
            }
        }
    });
}
