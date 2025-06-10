use crate::domain::finance::ports::FinanceMetrics;

/// An unimplemented example of an adapter to [FinanceMetrics].
#[derive(Debug, Clone)]
pub struct Prometheus;

impl Prometheus {
    pub fn new() -> Self {
        Self
    }
}

impl FinanceMetrics for Prometheus {
    async fn record_expense_creation_success(&self) {}

    async fn record_expense_creation_failure(&self) {}

    async fn record_expense_list_success(&self) {}
}
