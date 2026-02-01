use axum::extract::{Path, State};
use axum::Json;
use serde_json::Value;
use strum::IntoEnumIterator;
use tracing::info;
use validator::Validate;
use crate::{SharedState};
use crate::db::channel_repository::{get_all_channels, get_channel_by_name, insert_channel, save_channel};
use crate::dtos::channel::{ChannelCreateRequest, ChannelResponseDto, ChannelUpdateRequest};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{NewChannel, ProviderType};

pub async fn get_channel_by_id_handler(
    State(state): State<SharedState>,
    Path(_name): Path<String>,
) -> Result<AppResponse<ChannelResponseDto>, AppError> {
    let mut conn = state.pool.get().await?;

    let channel_option = get_channel_by_name(&mut conn, &_name).await?;

    if let Some(_channel) = channel_option {
        Ok(AppResponse::success_one("channel", _channel.into()))
    } else {
        Err(AppError::NotFound(format!("Channel doesn't exists for id '{}'", _name)))
    }
}

pub async fn create_channel_handler(
    State(state): State<SharedState>,
    Json(_create_request): Json<ChannelCreateRequest>,
) -> Result<AppResponse<ChannelResponseDto>, AppError> {
    _create_request.validate()?;
    let _config: Value = serde_json::from_str(&*_create_request.configuration)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    state.dispatcher.validate(&_create_request.provider_type, &_config).await?;

    info!("Creating channel with name: {}", _create_request.name);

    let mut conn = state.pool.get().await?;

    let existing = get_channel_by_name(&mut conn, &_create_request.name).await?;

    if existing.is_some() {
        return Err(AppError::Conflict(format!("Channel already exists for id '{}'", _create_request.name)));
    }

    let _new_channel = NewChannel {
        name: _create_request.name,
        provider_type: _create_request.provider_type,
        configuration: _config,
    };

    let inserted = insert_channel(&mut conn, _new_channel).await?;

    Ok(AppResponse::success_one("channel", inserted.into()))
}

pub async fn update_channel_handler(
    State(state): State<SharedState>,
    Path(_id): Path<String>,
    Json(_update_request): Json<ChannelUpdateRequest>,
) -> Result<AppResponse<ChannelResponseDto>, AppError> {
    _update_request.validate()?;
    let _config: Value = serde_json::from_str(&*_update_request.configuration)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    state.dispatcher.validate(&_update_request.provider_type, &_config).await?;

    info!("Updating channel with id: {}", _id);

    let mut conn = state.pool.get().await?;

    let mut _channel = get_channel_by_name(&mut conn, &_id)
        .await?
        .ok_or(AppError::NotFound(format!("Channel doesn't exists for id '{}'", _id)))?;

    _channel.provider_type = _update_request.provider_type;
    _channel.configuration = _config;

    let updated = save_channel(&mut conn, _channel).await?;
    Ok(AppResponse::success_one("channel", updated.into()))
}

pub async fn get_all_channels_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<ChannelResponseDto>>, AppError> {

    let mut conn = state.pool.get().await?;

    let channels = get_all_channels(&mut conn).await?;

    Ok(AppResponse::success_one("channels", channels.into_iter().map(Into::into).collect()))
}

pub async fn get_all_channel_providers_handler(
    State(_state): State<SharedState>,
) -> Result<AppResponse<Vec<ProviderType>>, AppError> {

    Ok(AppResponse::success_one("channel_providers", ProviderType::iter().collect() ))
}
