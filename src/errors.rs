use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bb8::RunError;
use chrono_tz::ParseError;
use diesel_async::pooled_connection::PoolError;
use strum_macros::Display;
use validator::{ValidationErrors};
use crate::jsend::AppResponse;

#[derive(Debug, Display)]
pub enum AppError {
    NotFound(String),
    DatabaseError(String),
    BadRequest(String),
    // Unauthorized(String),
    // Forbidden(String),
    Conflict(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(id) => (StatusCode::NOT_FOUND, format!("Resource not found: {}", id)),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("BadRequest: {}", msg)),
            // AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", msg)),
            // AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, format!("Forbidden: {}", msg)),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, format!("Conflict: {}", msg)),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", msg)),
        };
        (status, Json(AppResponse::<()>::error(message))).into_response()
    }
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

impl From<RunError<PoolError>> for AppError {
    fn from(error: RunError<PoolError>) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}
impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

impl From<ParseError> for AppError {
    fn from(error: ParseError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::BadRequest(error.to_string())
    }
}

// impl From<std::io::Error> for AppError {
//     fn from(error: std::io::Error) -> Self {
//         AppError::InternalError(error.to_string())
//     }
// }