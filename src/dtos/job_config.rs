use crate::validations::validate_name;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::{JobConfig, JobStageConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfigDto {
    pub app_name: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub enabled: bool,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub channel_ids: String,
}

impl From<JobConfig> for JobConfigDto {
    fn from(job_config: JobConfig) -> Self {
        Self {
            app_name: job_config.app_name,
            job_name: job_config.job_name,
            schedule: job_config.schedule,
            zone_id: job_config.zone_id,
            enabled: job_config.enabled,
            stages: job_config.stages,
            channel_ids: job_config.channel_ids,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct JobConfigCreateRequest {
    #[validate(custom(function = "validate_name"))]
    pub app_name: String,
    #[validate(custom(function = "validate_name"))]
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub channel_ids: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct JobConfigUpdateRequest {
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub channel_ids: String,
}