use chrono::{DateTime, Utc};
use diesel::{Queryable};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde_json::Value;
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStageConfig {
    pub name: String,
    pub start: Option<u64>,
    pub complete: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = job_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(app_name, job_name))]
pub struct JobConfig {
    pub app_name: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub enabled: bool,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub channel_ids: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = job_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewJobConfig {
    pub app_name: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub channel_ids: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::JobRunStatus")]
#[db_enum(value_style = "snake_case")]
pub enum JobRunStatus {
    InProgress,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = job_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobRun {
    pub id: Uuid,
    pub app_name: String,
    pub job_name: String,
    pub triggered_at: DateTime<Utc>,
    pub status: JobRunStatus,
    pub stages: diesel_json::Json<Vec<JobRunStage>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobRunStageStatus {
    Occurred,
    Failed,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRunStage {
    pub name: String,
    pub start_status: Option<JobRunStageStatus>,
    pub start_date_time: Option<DateTime<Utc>>,
    pub complete_status: Option<JobRunStageStatus>,
    pub complete_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = job_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewJobRun {
    pub app_name: String,
    pub job_name: String,
    pub triggered_at: DateTime<Utc>,
    pub status: JobRunStatus,
    pub stages: diesel_json::Json<Vec<JobRunStage>>,
}

#[derive(Display, Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq, EnumIter, Eq, Hash)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ProviderType")]
#[db_enum(value_style = "snake_case")]
pub enum ProviderType {
    GchatWebhook,
    EmailSmtp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(name))]
pub struct Channel {
    pub name: String,
    pub provider_type: ProviderType,
    pub configuration: Value,
    // pub configuration: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChannel {
    pub name: String,
    pub provider_type: ProviderType,
    // pub configuration: String,
    pub configuration: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = global_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Settings {
    pub id: i32,
    pub success_retention_days: i32,
    pub failure_retention_days: i32,
    pub maintenance_mode: bool,
    pub default_channels: String, // Coming from your MultiSelect join(",")
    pub max_stage_duration_hours: i32
}