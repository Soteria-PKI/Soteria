use rusqlite::Connection;

/// Handles the database connection for the backend.
pub fn database_connection() -> Result<(), String> {
  const SCHEMA: &str = include_str!("../../../../db/soteria-schema.sql");
  const TEST_DATA: &str = include_str!("../../../../db/data_test.sql");

  let conn: Connection = Connection::open("soteria-db").map_err(|e| e.to_string())?;
  conn.execute_batch(SCHEMA).map_err(|e| e.to_string())?;

  let command = conn.execute("SELECT version FROM sale WHERE id = 1", ());
  if command.is_err() {
    let _ = conn.execute_batch(TEST_DATA);
  }
  Ok(())
}
