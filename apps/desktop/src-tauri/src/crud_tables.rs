use crate::define_table_crud;

define_table_crud! {

    pub struct Category {
    name: String,
    description: Option<String>,
  }

  pub struct Customer {
    date_created: String,
    name: String,
    email: Option<String>,
    phone_number: Option<String>,
  }

  pub struct Discount {
    discount_type: Option<String>,
    amount: Option<i64>,
  }

  pub struct Employee {
    employee_number: String,
    name: String,
    date_created: String,
    email: Option<String>,
    phone_number: Option<String>,
    title: Option<String>,
  }

  pub struct Tag {
    name: Option<String>,
  }

  pub struct TaxClass {
    class: String,
    amount: i64,
  }

  pub struct Assembly {
    sku: Option<String>,
    name: String,
    cost: Option<f64>,
    price: Option<f64>,
    discount_id: Option<i64>,
    tax_class_id: Option<i64>,
  }

  pub struct Labour {
    sku: Option<String>,
    name: String,
    cost: Option<f64>,
    price: Option<f64>,
    duration: Option<i64>,
    discount_id: Option<i64>,
    tax_class_id: Option<i64>,
  }

  pub struct Vendor {
    name: String,
  }

  pub struct Item {
    sku: Option<String>,
    name: String,
    cost: Option<f64>,
    price: Option<f64>,
    quantity: Option<i64>,
    discount_id: Option<i64>,
    tax_class_id: Option<i64>,
    vendor_id: Option<i64>,
  }

  pub struct Box {
    sku: Option<String>,
    name: String,
    cost: Option<f64>,
    price: Option<f64>,
    quantity: Option<i64>,
    contained_item_id: Option<i64>,
    contained_quantity: Option<i64>,
    discount_id: Option<i64>,
    tax_class_id: Option<i64>,
    vendor_id: Option<i64>,
  }

  pub struct PurchaseOrder {
    order_id: String,
    name: String,
    vendor_id: Option<i64>,
    order_date: String,
    arrival_date: Option<String>,
    status: Option<String>,
    item_id: Option<i64>,
    box_id: Option<i64>,
  }

  pub struct Sale {
    timestamp: String,
    customer_id: i64,
    employee_id: i64,
    discount_id: Option<i64>,
    box_id: Option<i64>,
    item_id: Option<i64>,
    assembly_id: Option<i64>,
    labour_id: Option<i64>,
    service_order_id: Option<i64>,
  }

  pub struct Service {
    name: String,
    description: Option<String>,
    category_id: Option<i64>,
    employee_id: Option<i64>,
    item_id: Option<i64>,
    assembly_id: Option<i64>,
    labour_id: Option<i64>,
  }

  pub struct Appointment {
    appointment_number: Option<i64>,
    date_taken: String,
    date_occurring: String,
    approved: bool,
    service_id: Option<i64>,
    customer_id: Option<i64>,
  }

  pub struct ServiceOrder {
    date_created: String,
    date_due: Option<String>,
    employee_id: Option<i64>,
    customer_id: Option<i64>,
    notes_receipt: Option<String>,
    notes_internal: Option<String>,
    item_id: Option<i64>,
    labour_id: Option<i64>,
    assembly_id: Option<i64>,
  }

}
