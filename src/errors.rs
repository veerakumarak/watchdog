use aws_sdk_dynamodb::config::http::HttpResponse;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    DynamoError,
    DynamoDbError(String), // Or a more structured variant
    SerializationError,
    NotFound(String),
}

// Implement the From trait for automatic conversion via the `?` operator
impl From<SdkError<PutItemError, HttpResponse>> for AppError {
    fn from(err: SdkError<PutItemError, HttpResponse>) -> Self {
        // You can handle different types of SdkError here if needed
        // For simplicity, converting the error to a string is common
        AppError::DynamoDbError(err.to_string())
    }
}

// Convert from SDK errors
impl From<aws_sdk_dynamodb::Error> for AppError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        error!("DynamoDB Error: {}", err);
        AppError::DynamoError
    }
}

// Convert from serde_dynamo errors
impl From<serde_dynamo::Error> for AppError {
    fn from(err: serde_dynamo::Error) -> Self {
        error!("Serialization Error: {}", err);
        AppError::SerializationError
    }
}

// Implement IntoResponse to turn AppErrors into HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::DynamoError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            AppError::DynamoDbError(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", error)),
            AppError::SerializationError => (StatusCode::INTERNAL_SERVER_ERROR, "Data serialization error".to_string()),
            AppError::NotFound(id) => (StatusCode::NOT_FOUND, format!("Item not found: {}", id)),
        };
        (status, message).into_response()
    }
}