use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::models::ApiErrorResponse;

#[derive(Debug, Clone)]
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ApiErrorResponse {
            error: self.message,
        });

        (self.status, body).into_response()
    }
}

impl From<tonic::Status> for AppError {
    fn from(status: tonic::Status) -> Self {
        let http_status = match status.code() {
            tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
            tonic::Code::Unauthenticated => StatusCode::UNAUTHORIZED,
            tonic::Code::PermissionDenied => StatusCode::FORBIDDEN,
            tonic::Code::NotFound => StatusCode::NOT_FOUND,
            tonic::Code::AlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::BAD_GATEWAY,
        };

        Self::new(http_status, status.message())
    }
}
