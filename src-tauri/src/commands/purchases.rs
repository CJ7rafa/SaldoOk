use tauri::State;
use rusqlite::params;
use crate::models::{Purchase};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- PURCHASES ---

#[tauri::command]
pub fn get_purchases(state: State<'_, AppState>) -> Result<Vec<Purchase>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare(
        "SELECT p.id, p.supplier_id, p.invoice_number, p.issue_date, p.due_date, 
                p.total_amount, p.payment_type, p.status, p.notes, s.name as supplier_name
         FROM purchases p
         JOIN suppliers s ON p.supplier_id = s.id
         ORDER BY p.issue_date DESC"
    ).map_err(map_db_error)?;
    
    let iter = stmt.query_map([], |row| {
        Ok(Purchase {
            id: row.get(0)?,
            supplier_id: row.get(1)?,
            invoice_number: row.get(2)?,
            issue_date: row.get(3)?,
            due_date: row.get(4)?,
            total_amount: row.get(5)?,
            payment_type: row.get(6)?,
            status: row.get(7)?,
            notes: row.get(8)?,
            supplier_name: row.get(9)?,
        })
    }).map_err(map_db_error)?;
    
    let mut purchases = Vec::new();
    for purchase in iter {
        purchases.push(purchase.map_err(map_db_error)?);
    }
    
    Ok(purchases)
}

#[tauri::command]
pub fn create_purchase(state: State<'_, AppState>, purchase: Purchase) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO purchases (supplier_id, invoice_number, issue_date, due_date, total_amount, payment_type, status, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            purchase.supplier_id,
            purchase.invoice_number,
            purchase.issue_date,
            purchase.due_date,
            purchase.total_amount,
            purchase.payment_type,
            purchase.status,
            purchase.notes
        ],
    ).map_err(map_db_error)?;
    
    Ok(db.last_insert_rowid())
}

#[tauri::command]
pub fn update_purchase(state: State<'_, AppState>, purchase: Purchase) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.execute(
        "UPDATE purchases 
         SET supplier_id = ?1, invoice_number = ?2, issue_date = ?3, due_date = ?4, 
             total_amount = ?5, payment_type = ?6, status = ?7, notes = ?8
         WHERE id = ?9",
        params![
            purchase.supplier_id,
            purchase.invoice_number,
            purchase.issue_date,
            purchase.due_date,
            purchase.total_amount,
            purchase.payment_type,
            purchase.status,
            purchase.notes,
            purchase.id
        ],
    ).map_err(map_db_error)?;
    
    Ok(())
}

#[tauri::command]
pub fn delete_purchase(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    // Because of ON DELETE CASCADE on purchase_payments, payments will be automatically deleted.
    db.execute("DELETE FROM purchases WHERE id = ?1", params![id]).map_err(map_db_error)?;
    Ok(())
}