#[allow(unused_imports)] // Used in comment
use super::models::expense::ExpenseName;

use super::models::expense::{
    CreateExpenseError, CreateExpenseRequest, Expense, ListExpensesRequest, PaginationError,
};

/// `ExpenseRepository` represents a store of expense data.
pub trait ExpenseRepository: Clone + Send + Sync + 'static {
    /// Persist a new [Expense].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateExpenseError::Duplicate] if an [Expense] with the same [ExpenseName]
    ///   already exists.
    fn create_expense(
        &self,
        req: &CreateExpenseRequest,
    ) -> impl Future<Output = Result<Expense, CreateExpenseError>> + Send;

    /// Retrieve a list of [Expense].
    ///
    fn list_expenses(
        &self,
        req: &ListExpensesRequest,
    ) -> impl Future<Output = Result<Vec<Expense>, PaginationError>> + Send;
}
