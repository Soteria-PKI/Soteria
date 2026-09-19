// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Connection, Result};
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    let _ = database_connection();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn database_connection() -> Result<()>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let test_query = "SELECT * FROM TAG";
  let _ = conn.execute( &test_query, ());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
