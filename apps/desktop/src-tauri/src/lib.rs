// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Connection, Result, params};
use std::{fs, result};

#[tauri::command]
fn greet(name: &str) -> String {
    let _ = database_connection();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn insert_item(sku : Option<&str>, name : &str, cost : Option<f32>, price : Option<f32>, quantity : Option<i16>, discount_id : Option<i16>, tax_class_id : i16, vendor_id : Option<i16> ) -> Result<()>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let insert_query: &str = "INSERT INTO item (sku,name,cost,price,quantity,discount_id,tax_class_id,vendor_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
  let _ = conn.execute(&insert_query,params![sku,name,cost,price,quantity,discount_id,tax_class_id,vendor_id])?;
   Ok(())
}

#[tauri::command]
fn delete_item(id : i32) -> Result<usize>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let delete_query: &str = "DELETE FROM item WHERE id = ?1";
  let result = conn.execute(&delete_query,params![id]);

  result
    
  
}

#[tauri::command]
fn select_all_items() -> Result<usize>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let search_query: &str = "SELECT * item";
  let result = conn.execute(&search_query,());

  result
    
  
}

#[tauri::command]
fn select_item(id : i32) -> Result<usize>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let search_query: &str = "SELECT * item WHERE id = ?1";
  let result = conn.execute(&search_query,params![id]);

  result
    
  
}

#[tauri::command]
fn update_item(id : i32,sku : Option<&str>, name : &str, cost : Option<f32>, price : Option<f32>, quantity : Option<i16>, discount_id : Option<i16>, tax_class_id : i16, vendor_id : Option<i16>) -> Result<usize>{
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let update_query: &str = "UPDATE item
   SET sku = ?1, name = ?2, cost = ?3, price = ?4, quantity = ?5, discount_id = ?6, tax_class_id = ?7, vendor_id = ?8
   WHERE id = ?9";
  let result = conn.execute(&update_query,params![sku,name,cost,price,quantity,discount_id,tax_class_id,vendor_id,id]);

  result
    
  
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
