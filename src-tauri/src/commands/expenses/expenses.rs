use tauri::State;
use rusqlite::params;
use crate::models::{Expense};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_expense(state: State<'_, AppState>, expense: Expense) -> Result<i64, String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "INSERT INTO expenses (spender_id, category_id, amount, expense_date, notes)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            expense.spender_id,
            expense.category_id,
            expense.amount,
            expense.expense_date,
            expense.notes,
        ],
    ).map_err(map_db_error)?;

    Ok(db.last_insert_rowid())
}

// --- READ ALL --- //

#[tauri::command]
pub fn get_expenses(state: State<'_, AppState>) -> Result<Vec<Expense>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, spender_id, category_id, amount, expense_date, notes FROM expenses ORDER BY expense_date DESC").map_err(map_db_error)?;
    
    let iter = stmt.query_map([], |row| {
        Ok(Expense {
            id: row.get(0)?,
            spender_id: row.get(1)?,
            category_id: row.get(2)?,
            amount: row.get(3)?,
            expense_date: row.get(4)?,
            notes: row.get(5)?,
        })
    }).map_err(map_db_error)?;
    
    let mut expenses = Vec::new();
    for expense in iter {
        expenses.push(expense.map_err(map_db_error)?);
    }
    
    Ok(expenses)
}

// --- READ BY ID --- //

#[tauri::command]
pub fn get_expense_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Expense>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, spender_id, category_id, amount, expense_date, notes FROM expenses WHERE id = ?1").map_err(map_db_error)?;
    let mut rows = stmt.query([id]).map_err(map_db_error)?;
    
    if let Some(row) = rows.next().map_err(map_db_error)? {
        Ok(Some(Expense {
            id: row.get(0).map_err(map_db_error)?,
            spender_id: row.get(1).map_err(map_db_error)?,
            category_id: row.get(2).map_err(map_db_error)?,
            amount: row.get(3).map_err(map_db_error)?,
            expense_date: row.get(4).map_err(map_db_error)?,
            notes: row.get(5).map_err(map_db_error)?,
        }))
    } else {
        Ok(None)
    }
}

// --- UPDATE --- //

#[tauri::command]
pub fn update_expense(state: State<'_, AppState>, expense: Expense) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE expenses SET spender_id = ?1, category_id = ?2, amount = ?3, expense_date = ?4, notes = ?5 WHERE id = ?6",
        params![
            expense.spender_id,
            expense.category_id,
            expense.amount,
            expense.expense_date,
            expense.notes,
            expense.id
        ],
    ).map_err(map_db_error)?;

    Ok(())
}

// --- DELETE --- //

#[tauri::command]
pub fn delete_expense(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "DELETE FROM expenses WHERE id = ?1", 
        params![id],
    ).map_err(map_db_error)?;
    
    Ok(())
}