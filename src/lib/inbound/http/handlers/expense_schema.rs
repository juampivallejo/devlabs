use serde::Deserialize;

use crate::domain::finance::models::expense::CreateExpenseRequest;
use crate::domain::finance::models::expense::ExpenseNameEmptyError;

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
        Ok(CreateExpenseRequest::new(&self.name))?
    }
}
