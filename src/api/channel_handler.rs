use axum::extract::{Path, State};
use axum::Json;
use strum::IntoEnumIterator;
use tracing::info;
use crate::{SharedState};
use crate::db::channel_repository::{get_all_channels, get_channel_by_id, insert_channel, save_channel};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::{Channel, NewChannel, ProviderType};

pub async fn get_channel_by_id_handler(
    State(state): State<SharedState>,
    Path(_id): Path<String>,
) -> Result<AppResponse<Channel>, AppError> {
    let mut conn = state.pool.get().await?;

    let channel_option = get_channel_by_id(&mut conn, &_id).await?;

    if let Some(_channel) = channel_option {
        Ok(AppResponse::success_one("channel", _channel))
    } else {
        Err(AppError::NotFound(format!("Channel doesn't exists for id '{}'", _id)))
    }
}

pub async fn create_channel_handler(
    State(state): State<SharedState>,
    Json(_channel): Json<NewChannel>,
) -> Result<AppResponse<Channel>, AppError> {
    info!("Creating channel with id: {}", _channel.id);

    let mut conn = state.pool.get().await?;

    let existing = get_channel_by_id(&mut conn, &_channel.id).await?;

    if existing.is_some() {
        return Err(AppError::Conflict(format!("Channel already exists for id '{}'", _channel.id)));
    }

    let inserted = insert_channel(&mut conn, _channel).await?;

    Ok(AppResponse::success_one("channel", inserted))
}

pub async fn update_channel_handler(
    State(state): State<SharedState>,
    Path(_id): Path<String>,
    Json(_channel): Json<Channel>,
) -> Result<AppResponse<Channel>, AppError> {
    info!("Updating channel for id: {}", _id);

    if _id != _channel.id {
        return Err(AppError::BadRequest("invalid id provided".into()));
    }

    let mut conn = state.pool.get().await?;

    // ToDo - update update code
    let updated = save_channel(&mut conn, _channel).await?;

    Ok(AppResponse::success_one("channel", updated))
}

pub async fn get_all_channels_handler(
    State(state): State<SharedState>,
) -> Result<AppResponse<Vec<Channel>>, AppError> {

    let mut conn = state.pool.get().await?;

    let channels = get_all_channels(&mut conn).await?;

    Ok(AppResponse::success_one("channels", channels))
}

pub async fn get_all_channel_providers_handler(
    State(_state): State<SharedState>,
) -> Result<AppResponse<Vec<ProviderType>>, AppError> {

    Ok(AppResponse::success_one("channel_providers", ProviderType::iter().collect() ))
}
