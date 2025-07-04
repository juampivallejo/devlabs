use std::fmt::{Display, Formatter};

use derive_more::From;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expense {
    id: Uuid,
    name: ExpenseName,
}

impl Expense {
    pub fn new(id: Uuid, name: ExpenseName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &ExpenseName {
        &self.name
    }
}

/// A validated and formatted name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpenseName(String);

#[derive(Clone, Debug, Error)]
#[error("expense name cannot be empty")]
pub struct ExpenseNameEmptyError;

impl ExpenseName {
    pub fn new(raw: &str) -> Result<Self, ExpenseNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(ExpenseNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Display for ExpenseName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The fields required by the domain to create an [Expense].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateExpenseRequest {
    name: ExpenseName,
}

impl CreateExpenseRequest {
    pub fn new(name: &str) -> Result<Self, ExpenseNameEmptyError> {
        let name = ExpenseName::new(name)?;
        Ok(Self { name })
    }
    pub fn name(&self) -> &ExpenseName {
        &self.name
    }
}

/// The fields required by the domain to create an [Expense].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ListExpensesRequest {
    page: u32,
    size: u32,
}

impl ListExpensesRequest {
    pub fn new(page: u32, size: u32) -> Result<Self, PaginationError> {
        if page == 0 || size == 0 {
            Err(PaginationError::InvalidPage { page, size })
        } else {
            Ok(Self { page, size })
        }
    }
}

#[derive(Debug, Error)]
pub enum CreateExpenseError {
    #[error("expense with name {name} already exists")]
    Duplicate { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("Invalid page {page} or size {size}")]
    InvalidPage { page: u32, size: u32 },
    #[error("Page not found: {page}")]
    PageNotFound { page: u32 },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
