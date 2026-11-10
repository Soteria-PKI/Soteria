/// Database modules.
pub mod db {
  /// Database connection handling.
  pub mod database_connection;

  /// Table modules.
  pub mod tables {
    /// Table definitions.
    pub mod crud_tables;

    /// CRUD definition macro.
    pub mod define_table_crud;
  }
}

/// Utility modules.
pub mod util {
  /// Tauri handler generator helper macro.
  pub mod tauri_handler;
}
