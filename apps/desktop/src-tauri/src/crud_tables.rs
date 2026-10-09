use crate::define_table_crud;

define_table_crud! {
  pub struct TaxClass {
    class: String,
    amount: i64,
  }
}
