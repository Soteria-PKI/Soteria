use rusqlite::{Connection, Error, Row, Statement};

#[derive(Debug, serde::Serialize)]
pub struct TaxClass {
  id: u32,
  class: String,
  amount: u32,
}

#[tauri::command]
pub fn create_tax_class(tax_class: String, amount: u32) -> Result<i64, String> {
  let conn = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  conn.execute(
    "INSERT INTO TAX_CLASS (class, amount) VALUES (?1, ?2);",
    (&tax_class, &amount),
  );
  Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn select_tax_class(id: u32) -> Result<TaxClass, String> {
  let conn = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  let query = "SELECT * from TAX_CLASS WHERE id = ?1";
conn.query_one(query, [&id],|row| row.get(0));

    Ok(TaxClass{})
}

    

}

#[tauri::command]
pub fn update_tax_class() {}

//#[tauri::command]
//pub fn command_delete_tax_class() {}
