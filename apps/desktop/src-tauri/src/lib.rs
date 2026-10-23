pub mod backend;
pub use backend::db::{database_connection::database_connection, tables::crud_tables};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|_| {
      database_connection()?;
      Ok(())
    })
    .invoke_handler(tauri_handler! {
      appointment, assembly, category, customer, discount, employee, item, item_box,
      labour, purchase_order, service_order, sale, service, tag, tax_class, vendor
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
