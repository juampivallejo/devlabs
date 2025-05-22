use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    domain::finance::models::expense::{CreateExpenseError, ExpenseNameEmptyError},
    inbound::http::responses::ApiResponseBody,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

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

impl From<ExpenseNameEmptyError> for ApiError {
    fn from(_: ExpenseNameEmptyError) -> Self {
        Self::UnprocessableEntity("expense name cannot be empty".to_string())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

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
