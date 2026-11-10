use rusqlite::Connection;
use std::path::Path;

/// Handles the database connection for the backend.
pub fn connect_database(database_path: &Path) -> Result<(), String> {
  const SCHEMA: &str = include_str!("../../../../db/soteria-schema.sql");
  const TEST_DATA: &str = include_str!("../../../../db/data_test.sql");

  let conn: Connection = Connection::open(database_path).map_err(|e| e.to_string())?;
  conn.execute_batch(SCHEMA).map_err(|e| e.to_string())?;

  let command = conn.execute("SELECT version FROM sale WHERE id = 1", ());
  if command.is_err() {
    let _ = conn.execute_batch(TEST_DATA);
  }
  Ok(())
}
