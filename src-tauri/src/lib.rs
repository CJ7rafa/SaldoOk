mod models;
mod commands;

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::fs;
use std::sync::Mutex;
use tauri::Manager;

// Define migrations
fn get_migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(include_str!("../migrations/01_initial.sql")),
        M::up(include_str!("../migrations/02_add_supplier_color.sql")),
        M::up(include_str!("../migrations/03_add_all_system_expenses.sql")),
        M::up(include_str!("../migrations/04_drop_created_at_from_expenses.sql")),
        M::up(include_str!("../migrations/05_add_all_system_entry.sql")),
        M::up(include_str!("../migrations/06_add_house_shop_savings.sql")),
    ])
}

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(commands::get_handlers())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Initialize database
      let mut app_data_dir = app.handle().path().app_data_dir().expect("Failed to get app data dir");
      
      // Ensure the directory exists
      fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
      
      app_data_dir.push("database.sqlite");
      let mut conn = Connection::open(&app_data_dir).expect("Failed to open database");
      
      // Run migrations
      let migrations = get_migrations();
      migrations.to_latest(&mut conn).expect("Failed to run migrations");
      
      // Store connection in Tauri state
      app.manage(AppState {
          db: Mutex::new(conn),
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
