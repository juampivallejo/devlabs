use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    domain::finance::models::expense::{CreateExpenseError, ExpenseNameEmptyError},
    inbound::http::responses::ApiResponseBody,
};

/// Represents errors that can occur in the API layer.
///
/// This enum is used to map domain and infrastructure errors to HTTP responses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    /// Internal server error (HTTP 500).
    InternalServerError(String),
    /// Unprocessable entity error (HTTP 422).
    UnprocessableEntity(String),
}

/// Converts `CreateExpenseError` into an `ApiError`.
impl From<CreateExpenseError> for ApiError {
    fn from(e: CreateExpenseError) -> Self {
        match e {
            CreateExpenseError::Duplicate { name } => {
                Self::UnprocessableEntity(format!("expense with name {} already exists", name))
            }
            CreateExpenseError::Unknown(cause) => {
                tracing::error!("{:?}\n", cause);
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

/// Converts `ExpenseNameEmptyError` into an `ApiError`.
impl From<ExpenseNameEmptyError> for ApiError {
    fn from(_: ExpenseNameEmptyError) -> Self {
        Self::UnprocessableEntity("expense name cannot be empty".to_string())
    }
}

/// Converts `anyhow::Error` into an `ApiError`.
impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

/// Converts an `ApiError` into an HTTP response.
///
/// Maps the error variant to the appropriate HTTP status code and error message.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}
