use axum::extract::State;
use axum::Json;
use serde_dynamo::to_item;
use tracing::info;
use crate::{AppState};
use crate::errors::AppError;
use crate::jsend::AppResponse;
use crate::models::JobConfig;

pub async fn create_config(
    State(state): State<AppState>,
    Json(config): Json<JobConfig>,
) -> Result<AppResponse<()>, AppError> {
    info!("Updating config for job: {}", config.job_name);
    let item = to_item(config)?;

    state.db.put_item()
        .table_name(state.config_table)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(AppResponse::success())
}

pub async fn update_config(
    State(state): State<AppState>,
    Json(config): Json<JobConfig>,
) -> Result<AppResponse<()>, AppError> {
    info!("Updating config for job: {}", config.job_name);
    let item = to_item(config)?;

    state.db.put_item()
        .table_name(state.config_table)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(AppResponse::success())
}
