use tauri::State;
use rusqlite::params;
use crate::models::{Spender};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_spender(state: State<'_, AppState>, name: String) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    let trimmed_name = name.trim();
    
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM spenders WHERE LOWER(name) = LOWER(?1)",
        params![trimmed_name],
        |row| row.get(0),
    ).map_err(map_db_error)?;
    
    if count > 0 {
        return Err(format!("El gastador/persona '{}' ya existe.", trimmed_name));
    }

    db.execute(
        "INSERT INTO spenders (name, active) VALUES (?1, 1)",
        params![trimmed_name],
    ).map_err(map_db_error)?;
    
    Ok(db.last_insert_rowid())
}

// --- READ ALL --- //

#[tauri::command]
pub fn get_spenders(state: State<'_, AppState>) -> Result<Vec<Spender>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, name, active FROM spenders ORDER BY name ASC").map_err(map_db_error)?;
    
    let iter = stmt.query_map([], |row| {
        Ok(Spender {
            id: row.get(0)?,
            name: row.get(1)?,
            active: row.get(2)?,
        })
    }).map_err(map_db_error)?;
    
    let mut spenders = Vec::new();
    for spender in iter {
        spenders.push(spender.map_err(map_db_error)?);
    }
    Ok(spenders)
}

// --- READ BY ID --- //

#[tauri::command]
pub fn get_spender_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Spender>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, name, active FROM spenders WHERE id = ?1").map_err(map_db_error)?;
    let mut rows = stmt.query([id]).map_err(map_db_error)?;
    
    if let Some(row) = rows.next().map_err(map_db_error)? {
        Ok(Some(Spender {
            id: row.get(0).map_err(map_db_error)?,
            name: row.get(1).map_err(map_db_error)?,
            active: row.get(2).map_err(map_db_error)?,
        }))
    } else {
        Ok(None)
    }
}

// --- UPDATE --- //

#[tauri::command]
pub fn update_spender(state: State<'_, AppState>, spender: Spender) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE spenders SET name = ?1, active = ?2 WHERE id = ?3",
        params![
            spender.name.trim(),
            spender.active,
            spender.id,
        ],
    ).map_err(map_db_error)?;

    Ok(())
}

// --- DELETE --- //

#[tauri::command]
pub fn delete_spender(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "DELETE FROM spenders WHERE id = ?1", 
        params![id],
    ).map_err(map_db_error)?;
    
    Ok(())
}