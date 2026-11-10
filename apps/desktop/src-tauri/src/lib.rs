pub mod backend;
pub use backend::db::{database_connection::connect_database, tables::crud_tables};
use std::fs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      let mut database_path = app.path().app_data_dir()?;
      fs::create_dir_all(&database_path)?;

      database_path.push("soteria-db");
      connect_database(&database_path)?;

      Ok(())
    })
    .invoke_handler(tauri_handler! {
      appointment, assembly, category, customer, discount, employee, item, item_box,
      labour, purchase_order, service_order, sale, service, tag, tax_class, vendor
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
