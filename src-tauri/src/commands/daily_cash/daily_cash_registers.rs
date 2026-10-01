use tauri::State;
use rusqlite::params;
use crate::models::{DailyCashRegister};
use crate::AppState;

fn map_db_error(e: rusqlite::Error) -> String {
    format!("Database error: {}", e)
}

// --- CREATE --- //

#[tauri::command]
pub fn create_daily_cash_register(state: State<'_, AppState>, register: DailyCashRegister) -> Result<i64, String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "INSERT INTO daily_cash_registers (
            entry_date, 
            market_sales, market_vault_saved, 
            house_sales, house_shop_savings, 
            chain_income, vault_loan, 
            self_consumption, 
            notes
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            register.entry_date,
            register.market_sales,
            register.market_vault_saved,
            register.house_sales,
            register.house_shop_savings,
            register.chain_income,
            register.vault_loan,
            register.self_consumption,
            register.notes,
        ],
    ).map_err(map_db_error)?;

    Ok(db.last_insert_rowid())
}

// --- READ ALL --- //

#[tauri::command]
pub fn get_daily_cash_registers(state: State<'_, AppState>) -> Result<Vec<DailyCashRegister>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare(
        "SELECT id, entry_date, 
         market_sales, market_vault_saved, 
         house_sales, house_shop_savings, chain_income, vault_loan, 
         self_consumption, notes 
         FROM daily_cash_registers 
         ORDER BY entry_date DESC"
    ).map_err(map_db_error)?;
    
    let iter = stmt.query_map([], |row| {
        Ok(DailyCashRegister {
            id: row.get(0)?,
            entry_date: row.get(1)?,
            market_sales: row.get(2)?,
            market_vault_saved: row.get(3)?,
            house_sales: row.get(4)?,
            house_shop_savings: row.get(5)?,
            chain_income: row.get(6)?,
            vault_loan: row.get(7)?,
            self_consumption: row.get(8)?,
            notes: row.get(9)?,
        })
    }).map_err(map_db_error)?;
    
    let mut registers = Vec::new();
    for r in iter {
        registers.push(r.map_err(map_db_error)?);
    }
    
    Ok(registers)
}

// --- READ BY DATE --- //

#[tauri::command]
pub fn get_daily_cash_register_by_date(state: State<'_, AppState>, date: String) -> Result<Option<DailyCashRegister>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare(
        "SELECT id, entry_date, 
         market_sales, market_vault_saved, 
         house_sales, house_shop_savings, chain_income, vault_loan, 
         self_consumption, notes 
         FROM daily_cash_registers 
         WHERE entry_date = ?1"
    ).map_err(map_db_error)?;
    
    let mut iter = stmt.query_map([date], |row| {
        Ok(DailyCashRegister {
            id: row.get(0)?,
            entry_date: row.get(1)?,
            market_sales: row.get(2)?,
            market_vault_saved: row.get(3)?,
            house_sales: row.get(4)?,
            house_shop_savings: row.get(5)?,
            chain_income: row.get(6)?,
            vault_loan: row.get(7)?,
            self_consumption: row.get(8)?,
            notes: row.get(9)?,
        })
    }).map_err(map_db_error)?;
    
    if let Some(result) = iter.next() {
        Ok(Some(result.map_err(map_db_error)?))
    } else {
        Ok(None)
    }
}

// --- UPDATE --- //

#[tauri::command]
pub fn update_daily_cash_register(state: State<'_, AppState>, register: DailyCashRegister) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE daily_cash_registers SET 
            entry_date = ?1, 
            market_sales = ?2, 
            market_vault_saved = ?3, 
            house_sales = ?4, 
            house_shop_savings = ?5,
            chain_income = ?6, 
            vault_loan = ?7, 
            self_consumption = ?8, 
            notes = ?9 
         WHERE id = ?10",
        params![
            register.entry_date,
            register.market_sales,
            register.market_vault_saved,
            register.house_sales,
            register.house_shop_savings,
            register.chain_income,
            register.vault_loan,
            register.self_consumption,
            register.notes,
            register.id
        ],
    ).map_err(map_db_error)?;

    Ok(())
}

// --- DELETE --- //

#[tauri::command]
pub fn delete_daily_cash_register(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "DELETE FROM daily_cash_registers WHERE id = ?1", 
        params![id],
    ).map_err(map_db_error)?;
    
    Ok(())
}
