use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::responses::ApiResponseBody;

/// Wrapper type representing a successful API response.
///
/// Contains an HTTP status code and a JSON body with the response data.
#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

/// Implements equality comparison for `ApiSuccess`.
impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.0 == other.1.0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    /// Creates a new `ApiSuccess` with the given status code and data.
    pub fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    /// Converts the `ApiSuccess` into an HTTP response.
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}
