use anyhow::{Context, anyhow};
use sqlx::Executor;
use sqlx::Transaction;
use std::str::FromStr;
use tracing::Level;
use uuid::Uuid;

use crate::domain::finance::models::expense::ListExpensesRequest;
use crate::domain::finance::models::expense::PaginationError;
use crate::domain::finance::{
    models::expense::{CreateExpenseError, CreateExpenseRequest, Expense, ExpenseName},
    ports::ExpenseRepository,
};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: sqlx::SqlitePool,
}

impl Sqlite {
    /// Creates a new `Sqlite` instance with a connection pool to the specified database path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to the SQLite database.
    ///
    /// # Errors
    ///
    /// Returns an error if the database path is invalid or the connection cannot be established.
    pub async fn new(path: &str) -> anyhow::Result<Sqlite> {
        let pool = sqlx::SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }

    /// Saves an expense to the database.
    ///
    /// # Arguments
    ///
    /// * `_tx` - The database transaction.
    /// * `_name` - The name of the expense.
    ///
    /// # Returns
    ///
    /// Returns the generated UUID for the new expense.
    async fn save_expense(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        name: &ExpenseName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let span = tracing::span!(Level::DEBUG, "expense", expense_id = ?id);
        let _guard = span.enter();
        let id_as_string = id.to_string();
        let name = name.to_string();
        tracing::event!(
            Level::DEBUG,
            "Saving expense with ID: {} and name: {}",
            id_as_string,
            name
        );
        let query = sqlx::query!(
            "INSERT INTO expenses (id, name) VALUES ($1, $2)",
            id_as_string,
            name,
        );
        tx.execute(query).await?;

        tracing::event!(Level::DEBUG, "Expense Saved");
        Ok(id)
    }
}

/// Implementation of the `ExpenseRepository` trait for the `Sqlite` struct.
///
/// Provides methods to create and persist expenses in a SQLite database.
impl ExpenseRepository for Sqlite {
    /// Creates a new expense in the SQLite database.
    ///
    /// Starts a transaction, attempts to save the expense, and commits the transaction.
    /// Returns a `CreateExpenseError` if the operation fails or if a duplicate expense name exists.
    ///
    /// # Arguments
    ///
    /// * `req` - The request containing the expense name to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Expense)` if the expense is successfully created.
    /// * `Err(CreateExpenseError)` if there is a database error or a duplicate name.
    async fn create_expense(
        &self,
        req: &CreateExpenseRequest,
    ) -> Result<Expense, CreateExpenseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .unwrap_or_else(|e| panic!("failed to start SQLite transaction: {}", e));

        tracing::debug!("Transaction started");

        let expense_id = self.save_expense(&mut tx, req.name()).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateExpenseError::Duplicate {
                    name: req.name().to_string(),
                }
            } else {
                anyhow!(e)
                    .context(format!("failed to save expense with name {:?}", req.name()))
                    .into()
            }
        })?;
        tracing::info!("Expense saved with ID: {}", expense_id);

        tx.commit()
            .await
            .unwrap_or_else(|e| panic!("failed to commit SQLite transaction: {}", e));
        tracing::debug!("Transaction commited");

        Ok(Expense::new(expense_id, req.name().clone()))
    }

    async fn list_expenses(
        &self,
        req: &ListExpensesRequest,
    ) -> Result<Vec<Expense>, PaginationError> {
        Ok(Vec::new())
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}
