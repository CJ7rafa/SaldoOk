use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyCashRegister {
    pub id: Option<i64>,
    pub entry_date: String,
    
    // 1. MERCADO
    pub market_sales: f64,
    pub market_vault_saved: f64,
    
    // 2. CASA
    pub house_sales: f64,
    pub house_shop_savings: f64,
    
    // 3. OTROS INGRESOS MERCADO
    pub chain_income: f64,
    pub vault_loan: f64,
    
    // 4. RETIROS / AUTOCONSUMO
    pub self_consumption: f64,
    
    pub notes: Option<String>,
}
