use serde::Deserialize;

use crate::domain::finance::models::expense::CreateExpenseRequest;
use crate::domain::finance::models::expense::ExpenseNameEmptyError;
use crate::domain::finance::models::expense::ListExpensesRequest;
use crate::domain::finance::models::expense::PaginationError;

///
/// [CreateExpenseHttpRequestBody]
/// The HTTP Request body for creating an [Expense]
///
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateExpenseHttpRequestBody {
    pub name: String,
}

impl CreateExpenseHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    pub fn try_into_domain(self) -> Result<CreateExpenseRequest, ExpenseNameEmptyError> {
        CreateExpenseRequest::new(&self.name)
    }
}

///
/// [ListExpensesHttpRequestBody]
/// The HTTP Request with pagination for listing [Expense]
///
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationRequestQueryParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

impl PaginationRequestQueryParams {
    /// Converts the HTTP request body into a domain request.
    pub fn try_into_domain(self) -> Result<ListExpensesRequest, PaginationError> {
        let page = self.page.unwrap_or(1);
        let size = self.size.unwrap_or(10);
        ListExpensesRequest::new(page, size)
    }
}
