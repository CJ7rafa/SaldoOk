use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: Option<i64>,
    pub name: String,
    pub color: String,
    pub active: i32,
}