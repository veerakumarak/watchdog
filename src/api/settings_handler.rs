use axum::extract::{State};
use axum::Json;
use validator::Validate;
use crate::{SharedState};
use crate::db::settings_repository::{get_settings, save_settings};
use crate::dtos::settings::{SettingsResponseDto, SettingsUpdateRequest};
use crate::errors::AppError;
use crate::jsend::AppResponse;

pub async fn get_settings_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<SettingsResponseDto>, AppError> {
    let mut conn = state.pool.get().await?;

    let settings = get_settings(&mut conn).await?;

    Ok(AppResponse::success_one("settings", settings.into()))
}
pub async fn update_settings_handler(
    State(state): State<SharedState>,
    Json(_update_request): Json<SettingsUpdateRequest>,
) -> Result<AppResponse<SettingsResponseDto>, AppError> {
    _update_request.validate()?;

    let mut conn = state.pool.get().await?;

    let mut _settings = get_settings(&mut conn).await?;

    _settings.success_retention_days = _update_request.success_retention_days.unwrap();
    _settings.failure_retention_days = _update_request.failure_retention_days.unwrap();
    _settings.maintenance_mode = _update_request.maintenance_mode.unwrap();
    _settings.default_channels = _update_request.default_channels.unwrap();
    _settings.error_channels = _update_request.error_channels.unwrap();
    _settings.max_stage_duration_hours = _update_request.max_stage_duration_hours.unwrap();

    let updated = save_settings(&mut conn, _settings).await?;
    Ok(AppResponse::success_one("settings", updated.into()))
}
