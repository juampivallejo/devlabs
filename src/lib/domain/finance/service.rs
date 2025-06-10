use super::{
    models::expense::{CreateExpenseError, CreateExpenseRequest, Expense, ListExpensesRequest},
    ports::{ExpenseNotifier, ExpenseRepository, FinanceMetrics, FinanceService},
};
use anyhow::anyhow;

/// Canonical implementation of the [BlogService] port, through which the blog domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, M, N>
where
    R: ExpenseRepository,
    M: FinanceMetrics,
    N: ExpenseNotifier,
{
    repo: R,
    metrics: M,
    expense_notifier: N,
}

impl<R, M, N> Service<R, M, N>
where
    R: ExpenseRepository,
    M: FinanceMetrics,
    N: ExpenseNotifier,
{
    pub fn new(repo: R, metrics: M, expense_notifier: N) -> Self {
        Self {
            repo,
            metrics,
            expense_notifier,
        }
    }
}

impl<R, M, N> FinanceService for Service<R, M, N>
where
    R: ExpenseRepository,
    M: FinanceMetrics,
    N: ExpenseNotifier,
{
    /// Create the [Expense] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateExpenseError] returned by the [ExpenseRepository].
    async fn create_expense(
        &self,
        req: &CreateExpenseRequest,
    ) -> Result<Expense, CreateExpenseError> {
        let result = self.repo.create_expense(req).await;
        if result.is_err() {
            self.metrics.record_expense_creation_failure().await;
        } else {
            self.metrics.record_expense_creation_success().await;
            self.expense_notifier
                .expense_created(result.as_ref().unwrap())
                .await;
        }

        result
    }

    /// List [Expense].
    ///
    /// # Errors
    ///
    /// - Propagates any [ExpenseRepositoryError] returned by the [ExpenseRepository].
    async fn list_expenses(
        &self,
        req: &ListExpensesRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let result = self.repo.list_expenses(req).await;
        if result.is_ok() {
            self.metrics.record_expense_list_success().await;
        }
        result.map_err(|e| anyhow!("Failed to list expenses: {}", e))
    }
}
