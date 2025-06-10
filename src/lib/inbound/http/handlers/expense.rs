use axum::extract::Query;
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::domain::finance::ports::FinanceService;
use crate::inbound::http::server::AppState;
use crate::{
    domain::finance::models::expense::Expense,
    inbound::http::{api_error::ApiError, api_success::ApiSuccess},
};

use super::expense_schema::{CreateExpenseHttpRequestBody, PaginationRequestQueryParams};

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

///
/// `ExpenseResponseData`
/// The response body data field for [Expense] data.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExpenseResponseData {
    id: String,
    name: String,
}
impl From<&Expense> for ExpenseResponseData {
    fn from(expense: &Expense) -> Self {
        Self {
            id: expense.id().to_string(),
            name: expense.name().to_string(),
        }
    }
}

///
/// `ListItemsResponseData`
/// The generic response body data field for listing objects.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ListItemsResponseData<T> {
    items: Vec<T>,
}

impl<T> ListItemsResponseData<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

/// Create a new [Expense].
///
/// # Responses
///
/// - 201 Created: the [Expense] was successfully created.
/// - 422 Unprocessable entity: An [Expense] with the same name already exists.
pub async fn create_expense<FS: FinanceService>(
    State(state): State<AppState<FS>>,
    Json(body): Json<CreateExpenseHttpRequestBody>,
) -> Result<ApiSuccess<CreateExpenseResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .finance_service
        .create_expense(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref expense| ApiSuccess::new(StatusCode::CREATED, expense.into()))
}

/// List all [Expense].
///
/// # Responses
///
/// - 200 OK: the [Expense] list is returned.
/// - 404 Not Found: Page not found
/// - 422 Unprocessable entity: Invalid pagination parameters.
pub async fn list_expenses<FS>(
    State(state): State<AppState<FS>>,
    Query(query): Query<PaginationRequestQueryParams>,
) -> Result<ApiSuccess<ListItemsResponseData<ExpenseResponseData>>, ApiError>
where
    FS: FinanceService + Send + Sync + 'static,
{
    let domain_req = query.try_into_domain()?;

    state
        .finance_service
        .list_expenses(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|expenses| {
            ApiSuccess::new(
                StatusCode::OK,
                ListItemsResponseData::new(
                    expenses.iter().map(ExpenseResponseData::from).collect(),
                ),
            )
        })
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;

    use anyhow::anyhow;
    use uuid::Uuid;

    use crate::domain::finance::models::expense::{CreateExpenseError, ListExpensesRequest};
    use crate::domain::finance::models::expense::{CreateExpenseRequest, Expense, ExpenseName};
    use crate::domain::finance::ports::{ExpenseRepository, ExpenseRepositoryError};
    use crate::domain::finance::service::Service;
    use crate::outbound::email_client::EmailClient; // TODO: Use a mocked implementation once a
    // real email client is implemented.
    use crate::outbound::prometheus::Prometheus;

    use super::*;

    #[derive(Clone)]
    struct MockExpenseRepository {
        create_expense_result: Arc<std::sync::Mutex<Result<Expense, CreateExpenseError>>>,
        list_expenses_result: Arc<std::sync::Mutex<Result<Vec<Expense>, ExpenseRepositoryError>>>,
    }
    impl MockExpenseRepository {
        fn new() -> Self {
            Self {
                create_expense_result: Arc::new(std::sync::Mutex::new(Ok(Expense::new(
                    Uuid::new_v4(),
                    ExpenseName::new("Mock Expense").unwrap(),
                )))),
                list_expenses_result: Arc::new(std::sync::Mutex::new(Ok(vec![]))),
            }
        }
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
        async fn list_expenses(
            &self,
            _: &ListExpensesRequest,
        ) -> Result<Vec<Expense>, ExpenseRepositoryError> {
            let mut guard = self.list_expenses_result.lock();
            let mut result = Err(ExpenseRepositoryError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_expense_success() {
        let expense_name = ExpenseName::new("Angus").unwrap();
        let expense_id = Uuid::new_v4();
        let mut repo = MockExpenseRepository::new();
        let prometheus = Prometheus::new();
        let email_client = EmailClient::new();
        repo.create_expense_result = Arc::new(std::sync::Mutex::new(Ok(Expense::new(
            expense_id,
            expense_name.clone(),
        ))));
        let service = Service::new(repo.clone(), prometheus, email_client);

        let state = axum::extract::State(AppState {
            finance_service: Arc::new(service),
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_expenses_success() {
        let repo = MockExpenseRepository::new();
        let prometheus = Prometheus::new();
        let email_client = EmailClient::new();
        let service = Service::new(repo.clone(), prometheus, email_client);

        let state = axum::extract::State(AppState {
            finance_service: Arc::new(service),
        });
        let query = axum::extract::Query(PaginationRequestQueryParams {
            page: Some(1),
            size: Some(10),
        });
        let expected = ApiSuccess::new(StatusCode::OK, ListItemsResponseData::new(vec![]));

        let actual = list_expenses(state, query).await;
        assert!(
            actual.is_ok(),
            "expected list_expenses to succeed, but got {:?}",
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
