use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Purchase {
    pub id: Option<i64>,
    pub supplier_id: i64,
    pub invoice_number: Option<String>,
    pub issue_date: String,
    pub due_date: Option<String>,
    pub total_amount: f64,
    pub payment_type: String,
    pub status: String,
    pub notes: Option<String>,
    // Optional joined field for easier UI display
    pub supplier_name: Option<String>,
}