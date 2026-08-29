use tauri::State;
use rusqlite::params;
use crate::models::{PurchasePayment};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- PURCHASE PAYMENTS ---

#[tauri::command]
pub fn get_purchase_payments(state: State<'_, AppState>, purchase_id: i64) -> Result<Vec<PurchasePayment>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare(
        "SELECT id, purchase_id, payment_date, amount, payment_method, note 
         FROM purchase_payments 
         WHERE purchase_id = ?1 
         ORDER BY payment_date DESC"
    ).map_err(map_db_error)?;
    
    let iter = stmt.query_map(params![purchase_id], |row| {
        Ok(PurchasePayment {
            id: row.get(0)?,
            purchase_id: row.get(1)?,
            payment_date: row.get(2)?,
            amount: row.get(3)?,
            payment_method: row.get(4)?,
            note: row.get(5)?,
        })
    }).map_err(map_db_error)?;
    
    let mut payments = Vec::new();
    for payment in iter {
        payments.push(payment.map_err(map_db_error)?);
    }
    
    Ok(payments)
}

#[tauri::command]
pub fn create_purchase_payment(state: State<'_, AppState>, payment: PurchasePayment) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO purchase_payments (purchase_id, payment_date, amount, payment_method, note) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            payment.purchase_id,
            payment.payment_date,
            payment.amount,
            payment.payment_method,
            payment.note
        ],
    ).map_err(map_db_error)?;
    
    Ok(db.last_insert_rowid())
}

#[tauri::command]
pub fn update_purchase_payment(state: State<'_, AppState>, payment: PurchasePayment) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.execute(
        "UPDATE purchase_payments 
         SET payment_date = ?1, amount = ?2, payment_method = ?3, note = ?4
         WHERE id = ?5",
        params![
            payment.payment_date,
            payment.amount,
            payment.payment_method,
            payment.note,
            payment.id
        ],
    ).map_err(map_db_error)?;
    
    Ok(())
}

#[tauri::command]
pub fn delete_purchase_payment(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.execute("DELETE FROM purchase_payments WHERE id = ?1", params![id]).map_err(map_db_error)?;
    Ok(())
}
