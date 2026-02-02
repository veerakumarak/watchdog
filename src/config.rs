use std::env;
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub postgres_url: String,
    pub scheduler_initial_delay_seconds: u64,
    pub scheduler_fixed_delay_seconds: u64,
    pub error_channel_ids: String,
    pub grace_time_seconds: i64,
}

pub fn from_env() -> Config {
    dotenv().ok();
    Config { 
        postgres_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        scheduler_initial_delay_seconds: 2,
        scheduler_fixed_delay_seconds: 30,
        error_channel_ids: env::var("ERROR_CHANNEL_IDS").expect("ERROR_CHANNEL_IDS must be set"),
        grace_time_seconds: 5
    }
}