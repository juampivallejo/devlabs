use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::inbound::http::server::AppState;
use crate::{
    domain::finance::{models::expense::Expense, ports::ExpenseRepository},
    inbound::http::{api_error::ApiError, api_success::ApiSuccess},
};

use super::expense_schema::CreateExpenseHttpRequestBody;

///
/// `CreateExpenseResponseData`
/// The response body data field for successful [Expense] creation.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateExpenseResponseData {
    id: String,
}

impl From<&Expense> for CreateExpenseResponseData {
    fn from(expense: &Expense) -> Self {
        Self {
            id: expense.id().to_string(),
        }
    }
}

/// Create a new [Expense].
///
/// # Responses
///
/// - 201 Created: the [Expense] was successfully created.
/// - 422 Unprocessable entity: An [Expense] with the same name already exists.
pub async fn create_expense<PR: ExpenseRepository>(
    State(state): State<AppState<PR>>,
    Json(body): Json<CreateExpenseHttpRequestBody>,
) -> Result<ApiSuccess<CreateExpenseResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .expense_repo
        .create_expense(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref expense| ApiSuccess::new(StatusCode::CREATED, expense.into()))
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;

    use anyhow::anyhow;
    use uuid::Uuid;

    use crate::domain::finance::models::expense::CreateExpenseError;
    use crate::domain::finance::models::expense::{CreateExpenseRequest, Expense, ExpenseName};
    use crate::domain::finance::ports::ExpenseRepository;

    use super::*;

    #[derive(Clone)]
    struct MockExpenseRepository {
        create_expense_result: Arc<std::sync::Mutex<Result<Expense, CreateExpenseError>>>,
    }

    impl ExpenseRepository for MockExpenseRepository {
        async fn create_expense(
            &self,
            _: &CreateExpenseRequest,
        ) -> Result<Expense, CreateExpenseError> {
            let mut guard = self.create_expense_result.lock();
            let mut result = Err(CreateExpenseError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_expense_success() {
        let expense_name = ExpenseName::new("Angus").unwrap();
        let expense_id = Uuid::new_v4();
        let repo = MockExpenseRepository {
            create_expense_result: Arc::new(std::sync::Mutex::new(Ok(Expense::new(
                expense_id,
                expense_name.clone(),
            )))),
        };
        let state = axum::extract::State(AppState {
            expense_repo: Arc::new(repo),
        });
        let body = axum::extract::Json(CreateExpenseHttpRequestBody {
            name: expense_name.to_string(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateExpenseResponseData {
                id: expense_id.to_string(),
            },
        );

        let actual = create_expense(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_expense to succeed, but got {:?}",
            actual
        );

        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        )
    }
}
