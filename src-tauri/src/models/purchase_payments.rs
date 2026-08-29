use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchasePayment {
    pub id: Option<i64>,
    pub purchase_id: i64,
    pub payment_date: String,
    pub amount: f64,
    pub payment_method: String,
    pub note: Option<String>,
}
