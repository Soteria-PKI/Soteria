// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::Connection;
mod crud_tables;
mod define_table_crud;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

fn database_connection() -> Result<(), String> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");
  const TEST_DATA: &str = include_str!("../../db/data_test.sql");

  let conn = Connection::open("soteria-db").map_err(|e| e.to_string())?;

  conn.execute_batch(SCHEMA).map_err(|e| e.to_string())?;
  match conn.execute("SELECT version FROM sale WHERE id = 1", ()) {
    Ok(_) => (),
    Err(_) => {
      let _ = conn.execute_batch(TEST_DATA);
    }
  }
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let _ = database_connection();
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      greet,
      crud_tables::create_tax_class,
      crud_tables::read_tax_class,
      crud_tables::read_one_tax_class,
      crud_tables::update_tax_class,
      crud_tables::delete_tax_class
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
