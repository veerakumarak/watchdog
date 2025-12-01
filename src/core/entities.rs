use chrono::{DateTime, Utc};

pub struct JobConfig {
    pub application: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub enabled: bool,
    pub stages: Vec<JobStageConfig>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
pub struct JobStageConfig {
    pub name: String,
    pub relative_time_limit_secs: u64, // e.g., 300 (5 minutes)
}
