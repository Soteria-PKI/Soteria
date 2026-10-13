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

      crud_tables::create_service_order,
      crud_tables::read_service_order,
      crud_tables::read_one_service_order,
      crud_tables::update_service_order,
      crud_tables::delete_service_order,

      crud_tables::create_appointment,
      crud_tables::read_appointment,
      crud_tables::read_one_appointment,
      crud_tables::update_appointment,
      crud_tables::delete_appointment,

      crud_tables::create_service,
      crud_tables::read_service,
      crud_tables::read_one_service,
      crud_tables::update_service,
      crud_tables::delete_service,

      crud_tables::create_sale,
      crud_tables::read_sale,
      crud_tables::read_one_sale,
      crud_tables::update_sale,
      crud_tables::delete_sale,

      crud_tables::create_purchase_order,
      crud_tables::read_purchase_order,
      crud_tables::read_one_purchase_order,
      crud_tables::update_purchase_order,
      crud_tables::delete_purchase_order,

      crud_tables::create_item_box,
      crud_tables::read_item_box,
      crud_tables::read_one_item_box,
      crud_tables::update_item_box,
      crud_tables::delete_item_box,

      crud_tables::create_item,
      crud_tables::read_item,
      crud_tables::read_one_item,
      crud_tables::update_item,
      crud_tables::delete_item,

      crud_tables::create_vendor,
      crud_tables::read_vendor,
      crud_tables::read_one_vendor,
      crud_tables::update_vendor,
      crud_tables::delete_vendor,

      crud_tables::create_labour,
      crud_tables::read_labour,
      crud_tables::read_one_labour,
      crud_tables::update_labour,
      crud_tables::delete_labour,

      crud_tables::create_assembly,
      crud_tables::read_assembly,
      crud_tables::read_one_assembly,
      crud_tables::update_assembly,
      crud_tables::delete_assembly,

      crud_tables::create_tag,
      crud_tables::read_tag,
      crud_tables::read_one_tag,
      crud_tables::update_tag,
      crud_tables::delete_tag,

      crud_tables::create_employee,
      crud_tables::read_employee,
      crud_tables::read_one_employee,
      crud_tables::update_employee,
      crud_tables::delete_employee,

      crud_tables::create_discount,
      crud_tables::read_discount,
      crud_tables::read_one_discount,
      crud_tables::update_discount,
      crud_tables::delete_discount,

      crud_tables::create_customer,
      crud_tables::read_customer,
      crud_tables::read_one_customer,
      crud_tables::update_customer,
      crud_tables::delete_customer,

      crud_tables::create_category,
      crud_tables::read_category,
      crud_tables::read_one_category,
      crud_tables::update_category,
      crud_tables::delete_category,
      
      crud_tables::create_tax_class,
      crud_tables::read_tax_class,
      crud_tables::read_one_tax_class,
      crud_tables::update_tax_class,
      crud_tables::delete_tax_class,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
