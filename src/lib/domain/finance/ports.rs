use thiserror::Error;

#[allow(unused_imports)] // Used in comment
use super::models::expense::ExpenseName;

use super::models::expense::{
    CreateExpenseError, CreateExpenseRequest, Expense, ListExpensesRequest,
};

/// `FinanceService` is the public API for the finance domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait FinanceService: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Author].
    ///
    /// # Errors
    ///
    /// - [CreateExpenseError::Duplicate] if an [Expense] with the same [ExpenseName] already exists.
    fn create_expense(
        &self,
        req: &CreateExpenseRequest,
    ) -> impl Future<Output = Result<Expense, CreateExpenseError>> + Send;

    fn list_expenses(
        &self,
        req: &ListExpensesRequest,
    ) -> impl Future<Output = Result<Vec<Expense>, anyhow::Error>> + Send;
}

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
    ) -> impl Future<Output = Result<Vec<Expense>, ExpenseRepositoryError>> + Send;
}

#[derive(Debug, Error)]
pub enum ExpenseRepositoryError {
    #[error("Repository Timed out")]
    Timeout,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// `FinanceMetrics` describes an aggregator of finance-related metrics, such as a time-series
/// database.
pub trait FinanceMetrics: Send + Sync + Clone + 'static {
    /// Record a successful expense creation.
    fn record_expense_creation_success(&self) -> impl Future<Output = ()> + Send;

    /// Record an expense creation failure.
    fn record_expense_creation_failure(&self) -> impl Future<Output = ()> + Send;

    /// Record expenses retrieval success.
    fn record_expense_list_success(&self) -> impl Future<Output = ()> + Send;
}

/// `ExpenseNotifier` triggers notifications to expenses.
pub trait ExpenseNotifier: Send + Sync + Clone + 'static {
    fn expense_created(&self, expense: &Expense) -> impl Future<Output = ()> + Send;
}
