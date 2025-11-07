use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for a single stage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobStageConfig {
    pub name: String,
    pub relative_time_limit_secs: u64, // e.g., 300 (5 minutes)
}

/// The main configuration for a job, stored in the `job_configs` table.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobConfig {
    pub job_name: String, // Partition Key
    pub schedule: String, // e.g., "0 5 * * *" (for human reference)
    pub zone_id: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stages: Vec<JobStageConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobRun {
    pub run_id: String, // Partition Key
    pub job_name: String,
    pub start_time: u64, // Unix timestamp
    // Map of "stage_name" -> completion_timestamp
    pub completed_stages: HashMap<String, u64>,
    pub is_active: bool,
}
