use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::errors::AppError;
use crate::jsend::AppResponse;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, ())
}

pub async fn health_check2() -> Result<AppResponse<()>, AppError> {
    Ok(AppResponse::success())
}