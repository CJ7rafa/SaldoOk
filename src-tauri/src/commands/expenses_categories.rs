use tauri::State;
use rusqlite::params;
use crate::models::{ExpenseCategory};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_expense_category(state: State<'_, AppState>, category: ExpenseCategory) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO expense_categories (name, color, active) VALUES (?1, ?2, 1)",
        params![
            category.name,
            category.color,
        ],
    ).map_err(map_db_error)?;
    Ok(db.last_insert_rowid())
}

// --- READ ALL --- //

#[tauri::command]
pub fn get_all_expense_categories(state: State<'_, AppState>) -> Result<Vec<ExpenseCategory>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, name, color, active FROM expense_categories ORDER BY name ASC").map_err(map_db_error)?;
    let category_iter = stmt.query_map([], |row| {
        Ok(ExpenseCategory {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            active: row.get(3)?,
        })
    }).map_err(map_db_error)?;
    
    let mut categories = Vec::new();
    for category in category_iter {
        categories.push(category.map_err(map_db_error)?);
    }
    Ok(categories)
}

// --- READ BY ID --- //

#[tauri::command]
pub fn get_expense_category_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<ExpenseCategory>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, name, color, active FROM expense_categories WHERE id = ?1").map_err(map_db_error)?;
    let mut rows = stmt.query([id]).map_err(map_db_error)?;
    
    if let Some(row) = rows.next().map_err(map_db_error)? {
        Ok(Some(ExpenseCategory {
            id: row.get(0).map_err(map_db_error)?,
            name: row.get(1).map_err(map_db_error)?,
            color: row.get(2).map_err(map_db_error)?,
            active: row.get(3).map_err(map_db_error)?,
        }))
    } else {
        Ok(None)
    }
}

// --- UPDATE --- //

#[tauri::command]
pub fn update_expenses_categories(state: State<'_, AppState>, category: ExpenseCategory) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE expense_categories
        SET name = ?1, color = ?2, active = ?3
        WHERE id = ?4",
        params![
            category.name,
            category.color,
            category.active,
            category.id
        ],
    ).map_err(map_db_error)?;

    Ok(())
}

// --- DELETE --- //

#[tauri::command]
pub fn delete_expenses_categories(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "DELETE FROM expense_categories
        WHERE id = ?1", 
        params![id],
    ).map_err(map_db_error)?;
    
    Ok(())
}