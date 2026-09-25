  use rusqlite::{Connection, Result, Row, Statement};

  #[derive(Debug)]
  struct TaxClass {
    id: u32,
    class: String,
    amount: u32,
  }

  #[tauri::command]
  pub fn command_select_tax_class() {
        let _ = select_tax_class();
  }

  fn select_tax_class() -> Result<()> {
    let conn = Connection::open("soteria-db")?;
    let query = "SELECT * from TAX_CLASS";
    let mut stmt = conn.prepare(query)?;
    let tax_iter = stmt.query_map([], |row| {
      Ok(TaxClass {
        id:    row.get(0)?,
        class: row.get(1)?,
        amount:row.get(2)?,
      })
    })?;
    for class in tax_iter {
      println!("{:?}", class);
    }

    Ok(())
  }
