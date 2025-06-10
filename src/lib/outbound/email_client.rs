use crate::domain::finance::models::expense::Expense;
use crate::domain::finance::ports::ExpenseNotifier;

/// An unimplemented example of an adapter to [ExpenseNotifier].
#[derive(Debug, Clone)]
pub struct EmailClient;

impl EmailClient {
    pub fn new() -> Self {
        Self
    }
}

impl ExpenseNotifier for EmailClient {
    async fn expense_created(&self, _: &Expense) {}
}
