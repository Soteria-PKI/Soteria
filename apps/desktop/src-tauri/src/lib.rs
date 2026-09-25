// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Connection, Result, params};
use std::{fs, result};

#[derive(Debug)]
struct Item {
  id: i16,
  sku: Option<String>,
  name: String,
  cost: Option<f32>,
  price: Option<f32>,
  quantity: Option<i16>,
  discount_id: Option<i16>,
  tax_class_id: i16,
  vendor_id: Option<i16>,
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn insert_item(
  sku: Option<String>,
  name: String,
  cost: Option<f32>,
  price: Option<f32>,
  quantity: Option<i16>,
  discount_id: Option<i16>,
  tax_class_id: i16,
  vendor_id: Option<i16>,
) {
  println!("first rust");
  insert_not_tauri(
    sku,
    name,
    cost,
    price,
    quantity,
    discount_id,
    tax_class_id,
    vendor_id,
  )
  .map_err(|e| e.to_string());
}
fn insert_not_tauri(
  sku: Option<String>,
  name: String,
  cost: Option<f32>,
  price: Option<f32>,
  quantity: Option<i16>,
  discount_id: Option<i16>,
  tax_class_id: i16,
  vendor_id: Option<i16>,
) -> Result<()> {
  println!("second rust");
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let insert_query: &str = "INSERT INTO item (sku,name,cost,price,quantity,discount_id,tax_class_id,vendor_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
  let request = conn.execute(
    &insert_query,
    (
      sku.as_deref(),
      name,
      cost,
      price,
      quantity,
      discount_id,
      tax_class_id,
      vendor_id,
    ),
  )?;
  println!("Inserted {}", request);
  Ok(())
}

#[tauri::command]
fn delete_item(id: i32) -> Result<usize> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let delete_query: &str = "DELETE FROM item WHERE id = ?1";
  let result = conn.execute(&delete_query, params![id]);

  result
}

#[tauri::command]
fn select_all_items() {
  test_function();
}

fn test_function() -> Result<()> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let search_query: &str = "SELECT * FROM item";
  let mut stmt = conn.prepare(search_query)?;
  let item_iter = stmt.query_map([], |row| {
    Ok(Item {
      id: row.get(0)?,
      sku: row.get(1)?,
      name: row.get(2)?,
      cost: row.get(3)?,
      price: row.get(4)?,
      quantity: row.get(5)?,
      discount_id: row.get(6)?,
      tax_class_id: row.get(7)?,
      vendor_id: row.get(8)?,
    })
  })?;
  for item in item_iter {
    println!("Item : {:?}", item?)
  }
  Ok(())
}

#[tauri::command]
fn select_item(id: i32) -> Result<usize> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let search_query: &str = "SELECT * item WHERE id = ?1";
  let result = conn.execute(&search_query, params![id]);

  result
}

#[tauri::command]
fn update_item(
  id: i32,
  sku: Option<&str>,
  name: &str,
  cost: Option<f32>,
  price: Option<f32>,
  quantity: Option<i16>,
  discount_id: Option<i16>,
  tax_class_id: i16,
  vendor_id: Option<i16>,
) -> Result<usize> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  let update_query: &str = "UPDATE item
   SET sku = ?1, name = ?2, cost = ?3, price = ?4, quantity = ?5, discount_id = ?6, tax_class_id = ?7, vendor_id = ?8
   WHERE id = ?9";
  let result = conn.execute(
    &update_query,
    params![
      sku,
      name,
      cost,
      price,
      quantity,
      discount_id,
      tax_class_id,
      vendor_id,
      id
    ],
  );

  result
}

#[tauri::command]
fn database_connection() -> Result<()> {
  const SCHEMA: &str = include_str!("../../db/soteria-schema.sql");

  let conn = Connection::open("soteria-db")?;
  conn.execute_batch(SCHEMA)?;
  const test_data: &str = include_str!("../../db/data_test.sql");

  let _ = conn.execute_batch(&test_data);
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let _ = database_connection();
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      greet,
      insert_item,
      select_all_items
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
