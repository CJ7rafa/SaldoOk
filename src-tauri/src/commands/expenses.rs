use tauri::State;
use rusqlite::params;
use crate::models::{Expense};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_expenses(state: State<'_, AppState>, expense: Expense) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "INSERT INTO expenses (spender_id, category_id, amount, expense_date, notes)
        VALUES (?1, ?2, ?3, ?4, ?5)"
        params![
            expense.spender_id,
            expense.category_id,
            expense.amount,
            expense.expense_date,
            expense.notes,
        ],
    ).map_err(map_db_error)?;

    Ok(())
}

// --- READ --- //

#[tauri::command]
pub fn read_expenses(state: State<'_, AppState)