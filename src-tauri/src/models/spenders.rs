use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spender {
    pub id: Option<i64>,
    pub name: String,
    pub active: i32,
}