use std::env;
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub postgres_url: String,
}

pub fn from_env() -> Config {
    dotenv().ok();
    Config { 
        postgres_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set") 
    }
}