use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::{Settings};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SettingsResponseDto {
    pub success_retention_days: i32,
    pub failure_retention_days: i32,
    pub maintenance_mode: bool,
    pub default_channels: String,
    pub error_channels: String,
    pub max_stage_duration_hours: i32
}

impl From<Settings> for SettingsResponseDto {
    fn from(settings: Settings) -> Self {
        Self {
            success_retention_days: settings.success_retention_days,
            failure_retention_days: settings.failure_retention_days,
            maintenance_mode: settings.maintenance_mode,
            default_channels: settings.default_channels,
            error_channels: settings.error_channels,
            max_stage_duration_hours: settings.max_stage_duration_hours
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct SettingsUpdateRequest {
    pub success_retention_days: Option<i32>,
    pub failure_retention_days: Option<i32>,
    pub maintenance_mode: Option<bool>,
    pub default_channels: Option<String>,
    pub error_channels: Option<String>,
    pub max_stage_duration_hours: Option<i32>
}
