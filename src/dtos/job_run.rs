use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::{JobRun, JobRunStage, JobRunStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRunDto {
    pub id: String,
    pub app_name: String,
    pub job_name: String,
    pub triggered_at: DateTime<Utc>,
    pub status: JobRunStatus,
    pub stages: diesel_json::Json<Vec<JobRunStage>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<JobRun> for JobRunDto {
    fn from(job_run: JobRun) -> Self {
        Self {
            id: job_run.id.to_string(),
            app_name: job_run.app_name,
            job_name: job_run.job_name,
            triggered_at: job_run.triggered_at,
            status: job_run.status,
            stages: job_run.stages,
            created_at: job_run.created_at,
            updated_at: job_run.updated_at,
        }
    }
}
