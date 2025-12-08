use chrono::{DateTime, Utc};
use diesel::{Queryable};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde_json::Value;
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
#[diesel(primary_key(application, job_name))]
pub struct JobConfig {
    pub application: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub enabled: bool,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = job_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewJobConfig {
    pub application: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
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
    pub application: String,
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
    pub application: String,
    pub job_name: String,
    pub triggered_at: DateTime<Utc>,
    pub status: JobRunStatus,
    pub stages: diesel_json::Json<Vec<JobRunStage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub configuration: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChannel {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub configuration: Value,
}
