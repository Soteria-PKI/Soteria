use rusqlite::{Connection, Result, Row, Statement};

#[derive(Debug, serde::Serialize)]
struct TaxClass {
  id: u32,
  class: String,
  amount: u32,
}

#[tauri::command]
pub fn create_tax_class(class: String, amount: u32) -> Result<i64, String> {
  let conn = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  conn
    .execute(
      "INSERT INTO TAX_CLASS (class, amount) VALUES (?1, ?2);",
      (&class, &amount),
    )
    .map_err(|e| e.to_string());
  Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn command_select_tax_class() {
  dbg!("command_select_tax_class called");
  let _ = select_tax_class();
}

fn select_tax_class() -> Result<()> {
  dbg!("in command_select_tax_class");
  let conn = Connection::open("soteria-db")?;
  let query = "SELECT * from TAX_CLASS";
  let mut stmt = conn.prepare(query)?;
  let tax_iter = stmt.query_map([], |row| {
    Ok(TaxClass {
      id: row.get(0)?,
      class: row.get(1)?,
      amount: row.get(2)?,
    })
  })?;
  for class in tax_iter {
    println!("{:?}", class);
  }

  Ok(())
}

//#[tauri::command]
//pub fn command_update_tax_class() {}
//fn update_tax_class() -> Result<()>{Ok(())}

//#[tauri::command]
//pub fn command_delete_tax_class() {}
