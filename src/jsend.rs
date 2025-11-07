use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

/// JSend-style status enum
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Fail,
    Error,
}

/// JSend-style API response structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppResponse<T>
where T: Serialize,
{
    pub status: Status,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, T>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_data: Option<HashMap<String, serde_json::Value>>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    /// Creates an empty success response: `{"status":"success","data":{}}`
    pub fn success() -> Self {
        Self {
            status: Status::Success,
            reasons: None,
            data: Some(HashMap::new()),
            message: None,
            code: None,
            error_data: None,
        }
    }

    /// Creates a success response with a single key/value pair.
    pub fn success_one<K: Into<String>>(key: K, value: T) -> Self {
        let mut data = HashMap::new();
        data.insert(key.into(), value);
        Self {
            status: Status::Success,
            reasons: None,
            data: Some(data),
            message: None,
            code: None,
            error_data: None,
        }
    }

    /// Creates a success response with a map of data.
    pub fn success_map(data: HashMap<String, T>) -> Self {
        Self {
            status: Status::Success,
            reasons: None,
            data: Some(data),
            message: None,
            code: None,
            error_data: None,
        }
    }

    /// Creates a fail response with a message.
    pub fn fail_message<M: Into<String>>(message: M) -> Self {
        Self {
            status: Status::Fail,
            reasons: None,
            data: None,
            message: Some(message.into()),
            code: None,
            error_data: None,
        }
    }

    /// Creates a fail response with a single reason.
    pub fn fail_reason<K: Into<String>, M: Into<String>>(key: K, msg: M) -> Self {
        let mut reasons = HashMap::new();
        reasons.insert(key.into(), msg.into());
        Self {
            status: Status::Fail,
            reasons: Some(reasons),
            data: None,
            message: None,
            code: None,
            error_data: None,
        }
    }

    /// Creates a fail response with a map of reasons.
    pub fn fail_reasons(reasons: HashMap<String, String>) -> Self {
        Self {
            status: Status::Fail,
            reasons: Some(reasons),
            data: None,
            message: None,
            code: None,
            error_data: None,
        }
    }

    /// Creates an error response with a message.
    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            status: Status::Error,
            reasons: None,
            data: None,
            message: Some(message.into()),
            code: None,
            error_data: None,
        }
    }

    /// Creates an error response with message and code.
    pub fn error_with_code<M: Into<String>>(message: M, code: i32) -> Self {
        Self {
            status: Status::Error,
            reasons: None,
            data: None,
            message: Some(message.into()),
            code: Some(code),
            error_data: None,
        }
    }

    /// Creates an error response with message, code, and additional error data.
    pub fn error_with_data<M: Into<String>>(
        message: M,
        code: Option<i32>,
        error_data: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            status: Status::Error,
            reasons: None,
            data: None,
            message: Some(message.into()),
            code,
            error_data: Some(error_data),
        }
    }

    /// Helpers like is_success(), etc.
    pub fn is_success(&self) -> bool {
        self.status == Status::Success
    }
    pub fn is_fail(&self) -> bool {
        self.status == Status::Fail
    }
    pub fn is_error(&self) -> bool {
        self.status == Status::Error
    }
}

/// Allow direct return from Axum handlers
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = match self.status {
            Status::Success => StatusCode::OK,
            Status::Fail => StatusCode::BAD_REQUEST,
            Status::Error => {
                StatusCode::from_u16(self.code.unwrap_or(500) as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        (status_code, Json(self)).into_response()
    }
}
