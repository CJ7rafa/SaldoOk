use tauri::State;
use rusqlite::params;
use crate::models::{ExpenseCategory};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_expense_category()