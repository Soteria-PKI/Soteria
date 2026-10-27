/// Defines the Tauri handler for more easily exposing all 5 commands for each
/// table. This allows for more easily viewing the existing tables.
#[macro_export]
macro_rules! tauri_handler {
  ($($table_name: ident),+) => {
    pastey::paste! {
      tauri::generate_handler! [
        $(
          crud_tables::[<create_ $table_name>],
          crud_tables::[<read_ $table_name>],
          crud_tables::[<read_one_ $table_name>],
          crud_tables::[<update_ $table_name>],
          crud_tables::[<delete_ $table_name>],
        )+
      ]
    }
  };
}
