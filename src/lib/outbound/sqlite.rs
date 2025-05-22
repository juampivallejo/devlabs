use anyhow::{Context, anyhow};
use sqlx::Transaction;
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::finance::{
    models::expense::{CreateExpenseError, CreateExpenseRequest, Expense, ExpenseName},
    ports::ExpenseRepository,
};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: sqlx::SqlitePool,
}

impl Sqlite {
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

    async fn save_expense(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        name: &ExpenseName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let name = &name.to_string();
        // let query = sqlx::query!(
        //     "INSERT INTO expenses (id, name) VALUES (?, ?)",
        //     id_as_string,
        //     name,
        // );
        // tx.execute(query).await?;
        Ok(id)
    }
}

impl ExpenseRepository for Sqlite {
    async fn create_expense(
        &self,
        req: &CreateExpenseRequest,
    ) -> Result<Expense, CreateExpenseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .unwrap_or_else(|e| panic!("failed to start SQLite transaction: {}", e));

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

        tx.commit()
            .await
            .unwrap_or_else(|e| panic!("failed to commit SQLite transaction: {}", e));

        Ok(Expense::new(expense_id, req.name().clone()))
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
