use rusqlite::Connection;

/// Representation of a Tax Class
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxClass {
  id: i64,
  /// A legally defined item category specifying a tax percentage to apply to that item in a sale.
  class: String,
  /// The amount, in percentage, of tax to apply.
  amount: i64,
}

#[tauri::command]
pub fn create_tax_class(tax_class: String, amount: i64) -> Result<TaxClass, String> {
  let conn: Connection = Connection::open("soteria-db").map_err(|e| e.to_string())?;

  conn
    .execute(
      "INSERT INTO TAX_CLASS (class, amount) VALUES (?1, ?2);",
      (&tax_class, &amount),
    )
    .map_err(|e| e.to_string())?;

  let result_id = conn.last_insert_rowid();

  Ok(TaxClass {
    id: result_id,
    class: tax_class,
    amount,
  })
}

#[tauri::command]
pub fn select_all_tax_class() -> Result<Vec<TaxClass>, String> {
  let conn: Connection = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  let mut stmt = conn
    .prepare("SELECT * FROM tax_class;")
    .map_err(|e| e.to_string())?;
  let tax_class_iter = stmt
    .query_map([], |row| {
      Ok(TaxClass {
        id: row.get(0)?,
        class: row.get(1)?,
        amount: row.get(2)?,
      })
    })
    .map_err(|e| e.to_string())?;

  let result: Vec<TaxClass> = tax_class_iter
    // .collect::<Result<Vec<_>, _>> is used below to specify that the type to be retured
    // should be a Result<Vec<>, Err>, because technically it could also be interpreted as
    // a Vec<Result <TaxClass, Error>> at this stage
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;

  Ok(result)
}

#[tauri::command]
pub fn select_tax_class(id: i64) -> Result<TaxClass, String> {
  let conn: Connection = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  let response = conn
    .query_one("SELECT * FROM tax_class WHERE id = ?1", [&id], |row| {
      Ok(TaxClass {
        id: row.get(0)?,
        class: row.get(1)?,
        amount: row.get(2)?,
      })
    })
    .map_err(|e| e.to_string())?;
  Ok(response)
}

#[tauri::command]
pub fn update_tax_class(id: i64, tax_class: String, amount: i64) -> Result<TaxClass, String>{
  let conn: Connection = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  let query = "";
    let mut stmt = conn.prepare()


    Err(String::from("poop"))
}

//#[tauri::command]
//pub fn command_delete_tax_class() {}
