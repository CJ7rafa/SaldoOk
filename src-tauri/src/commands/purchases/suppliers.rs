use tauri::State;
use rusqlite::params;
use crate::models::{Supplier};
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

// --- UPDATE --- //

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
