pub mod backend;
pub use backend::db::{
  database_connection::connect_database,
  tables::{complete_purchase_order, crud_tables},
};
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
      std::env::set_current_dir(app.path().app_data_dir()?)?;

      Ok(())
    })
    .invoke_handler(tauri_handler! {
      appointment, assembly, category, customer, discount, employee, item, item_box,
      labour, purchase_order, service_order, sale, service, tag, tax_class, vendor;
      complete_purchase_order::read_complete_purchase_order,
      complete_purchase_order::read_one_complete_purchase_order,
      complete_purchase_order::create_complete_purchase_order,
      complete_purchase_order::update_complete_purchase_order,
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
