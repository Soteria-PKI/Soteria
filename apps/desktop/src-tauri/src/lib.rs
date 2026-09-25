// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod crud_tax_class;
use rusqlite::{Connection, Result, Row};


#[tauri::command]
fn greet(name: &str) -> String {
  let _ = database_connection();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test_select() {
  match execute_select() {
    Ok(row) => (),
    Err(err) => eprintln!("{:?}", err),
  }
}

fn execute_select() -> Result<()> {
  let conn = Connection::open("soteria-db")?;
  let _ = conn.query_row("SELECT version FROM database_version", [], |row| {
    println!("{:#?}", row);
    println!("{}", Row::get_unwrap::<usize,u32>(row,1));
    Ok(())
  })?;

  Ok(())
}

#[tauri::command]
fn database_connection() -> Result<()> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");
  const TEST_DATA: &str = include_str!("../../db/data_test.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  conn.execute_batch(TEST_DATA)?;
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet, test_select, crud_tax_class::select_tax_class])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
