use tauri::State;
use rusqlite::params;
use crate::models::{Supplier, Purchase, PurchasePayment};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- SUPPLIERS ---

#[tauri::command]
pub fn get_suppliers(state: State<'_, AppState>) -> Result<Vec<Supplier>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, name, active, color FROM suppliers ORDER BY name ASC").map_err(map_db_error)?;
    
    let iter = stmt.query_map([], |row| {
        Ok(Supplier {
            id: row.get(0)?,
            name: row.get(1)?,
            active: row.get(2)?,
            color: row.get(3)?,
        })
    }).map_err(map_db_error)?;
    
    let mut suppliers = Vec::new();
    for supplier in iter {
        suppliers.push(supplier.map_err(map_db_error)?);
    }
    
    Ok(suppliers)
}

#[tauri::command]
pub fn create_supplier(state: State<'_, AppState>, name: String, color: String) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    
    let trimmed_name = name.trim();
    
    // Check for existing supplier (case insensitive)
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM suppliers WHERE LOWER(name) = LOWER(?1)",
        params![trimmed_name],
        |row| row.get(0),
    ).map_err(map_db_error)?;
    
    if count > 0 {
        return Err(format!("El proveedor '{}' ya existe.", trimmed_name));
    }

    db.execute(
        "INSERT INTO suppliers (name, active, color) VALUES (?1, 1, ?2)",
        params![trimmed_name, color],
    ).map_err(map_db_error)?;
    
    Ok(db.last_insert_rowid())
}

#[tauri::command]
pub fn update_supplier(state: State<'_, AppState>, id: i64, name: String, active: i32, color: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    
    let trimmed_name = name.trim();
    
    // Check for existing supplier with same name but different id
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM suppliers WHERE LOWER(name) = LOWER(?1) AND id != ?2",
        params![trimmed_name, id],
        |row| row.get(0),
    ).map_err(map_db_error)?;
    
    if count > 0 {
        return Err(format!("Ya existe otro proveedor con el nombre '{}'.", trimmed_name));
    }

    db.execute(
        "UPDATE suppliers SET name = ?1, active = ?2, color = ?3 WHERE id = ?4",
        params![trimmed_name, active, color, id],
    ).map_err(map_db_error)?;
    
    Ok(())
}

#[tauri::command]
pub fn delete_supplier(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    
    // Verificar si el proveedor tiene compras
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM purchases WHERE supplier_id = ?1",
        params![id],
        |row| row.get(0),
    ).map_err(map_db_error)?;
    
    if count > 0 {
        return Err("No se puede eliminar el proveedor porque tiene compras asociadas. Considere desactivarlo.".into());
    }
    
    db.execute(
        "DELETE FROM suppliers WHERE id = ?1",
        params![id],
    ).map_err(map_db_error)?;
    
    Ok(())
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
