use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: Option<i64>,
    pub spender_id: i64,
    pub category_id: i64,
    pub amount: f64,
    pub expense_date: String,
    pub notes: Option<String>,
}