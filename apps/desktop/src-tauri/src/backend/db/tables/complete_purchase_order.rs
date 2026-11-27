//! Purchase orders together with their lines (the items and item boxes on
//! the order, read from the `junc_purchase_order_item` and
//! `junc_purchase_order_item_box` junction tables).

use super::crud_tables::{Item, ItemBox, PurchaseOrder};
use rusqlite::{Connection, Error, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

/// One item on a purchase order and the ordered quantity.
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PurchaseOrderItemLine {
  item: Item,
  quantity: i64,
}

/// One item box on a purchase order and the ordered quantity.
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PurchaseOrderItemBoxLine {
  item_box: ItemBox,
  quantity: i64,
}

/// A purchase order with all of its item and item box lines.
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CompletePurchaseOrder {
  purchase_order: PurchaseOrder,
  items: Vec<PurchaseOrderItemLine>,
  item_boxes: Vec<PurchaseOrderItemBoxLine>,
}

/// A line to write to a purchase order. `id` is the id of the item or of the
/// item box, depending on the list that contains the line.
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PurchaseOrderLineInput {
  id: i64,
  quantity: i64,
}

/// A purchase order and its complete list of lines, sent by the frontend to
/// create or update an order. The lines replace all existing lines.
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CompletePurchaseOrderInput {
  purchase_order: PurchaseOrder,
  items: Vec<PurchaseOrderLineInput>,
  item_boxes: Vec<PurchaseOrderLineInput>,
}

/// Reads all purchase orders with their lines.
#[tauri::command]
pub(crate) fn read_complete_purchase_order() -> Result<Vec<CompletePurchaseOrder>, String> {
  err_wrap(|| {
    let connection = Connection::open("soteria-db")?;
    let mut statement = connection.prepare(&format!("SELECT * FROM {}", PurchaseOrder::TABLE))?;
    let orders = statement
      .query_map([], PurchaseOrder::from_row)?
      .collect::<Result<Vec<_>, _>>()?;

    orders
      .into_iter()
      .map(|order| with_lines(&connection, order))
      .collect()
  })
}

/// Reads one purchase order with its lines. Returns `None` if no purchase
/// order has this id.
#[tauri::command]
pub(crate) fn read_one_complete_purchase_order(
  id: i64,
) -> Result<Option<CompletePurchaseOrder>, String> {
  err_wrap(|| {
    let connection = Connection::open("soteria-db")?;
    let order = connection
      .query_row(
        &format!("SELECT * FROM {} WHERE id = ?1", PurchaseOrder::TABLE),
        (id,),
        PurchaseOrder::from_row,
      )
      .optional()?;

    order
      .map(|order| with_lines(&connection, order))
      .transpose()
  })
}

/// Creates a purchase order and its lines in one transaction. The `id` of the
/// purchase order is ignored. Returns the id of the new purchase order.
#[tauri::command]
pub(crate) fn create_complete_purchase_order(
  order: CompletePurchaseOrderInput,
) -> Result<i64, String> {
  validate_lines(&order)?;
  err_wrap(|| {
    let mut connection = Connection::open("soteria-db")?;
    let transaction = connection.transaction()?;

    let id = order.purchase_order.insert(&transaction)?;
    insert_lines(&transaction, id, &order)?;

    transaction.commit()?;
    Ok(id)
  })
}

/// Updates a purchase order and replaces all of its lines in one transaction.
#[tauri::command]
pub(crate) fn update_complete_purchase_order(
  order: CompletePurchaseOrderInput,
) -> Result<(), String> {
  validate_lines(&order)?;
  err_wrap(|| {
    let mut connection = Connection::open("soteria-db")?;
    let transaction = connection.transaction()?;
    let id = order.purchase_order.id;

    if order.purchase_order.update(&transaction)? == 0 {
      return Err(Error::QueryReturnedNoRows);
    }
    transaction.execute(
      "DELETE FROM junc_purchase_order_item WHERE purchase_order_id = ?1",
      (id,),
    )?;
    transaction.execute(
      "DELETE FROM junc_purchase_order_item_box WHERE purchase_order_id = ?1",
      (id,),
    )?;
    insert_lines(&transaction, id, &order)?;

    transaction.commit()
  })
}

/// Reads the item and item box lines of `order`.
fn with_lines(
  connection: &Connection,
  order: PurchaseOrder,
) -> Result<CompletePurchaseOrder, Error> {
  // `line_quantity` is aliased because `item` and `item_box` also have a
  // `quantity` column (the quantity in stock).
  let mut statement = connection.prepare(
    "SELECT item.*, line.quantity AS line_quantity
     FROM junc_purchase_order_item AS line
     JOIN item ON item.id = line.item_id
     WHERE line.purchase_order_id = ?1
     ORDER BY item.name",
  )?;
  let items = statement
    .query_map((order.id,), |row| {
      Ok(PurchaseOrderItemLine {
        item: Item::from_row(row)?,
        quantity: row.get("line_quantity")?,
      })
    })?
    .collect::<Result<Vec<_>, _>>()?;

  let mut statement = connection.prepare(
    "SELECT item_box.*, line.quantity AS line_quantity
     FROM junc_purchase_order_item_box AS line
     JOIN item_box ON item_box.id = line.item_box_id
     WHERE line.purchase_order_id = ?1
     ORDER BY item_box.name",
  )?;
  let item_boxes = statement
    .query_map((order.id,), |row| {
      Ok(PurchaseOrderItemBoxLine {
        item_box: ItemBox::from_row(row)?,
        quantity: row.get("line_quantity")?,
      })
    })?
    .collect::<Result<Vec<_>, _>>()?;

  Ok(CompletePurchaseOrder {
    purchase_order: order,
    items,
    item_boxes,
  })
}

/// Inserts the lines of `order` for the purchase order `id`.
fn insert_lines(
  connection: &Connection,
  id: i64,
  order: &CompletePurchaseOrderInput,
) -> Result<(), Error> {
  let mut statement = connection.prepare(
    "INSERT INTO junc_purchase_order_item (purchase_order_id, item_id, quantity) VALUES (?1, ?2, ?3)",
  )?;
  for line in &order.items {
    statement.execute((id, line.id, line.quantity))?;
  }

  let mut statement = connection.prepare(
    "INSERT INTO junc_purchase_order_item_box (purchase_order_id, item_box_id, quantity) VALUES (?1, ?2, ?3)",
  )?;
  for line in &order.item_boxes {
    statement.execute((id, line.id, line.quantity))?;
  }
  Ok(())
}

/// Rejects quantities below 1 and an item or item box that occurs twice.
fn validate_lines(order: &CompletePurchaseOrderInput) -> Result<(), String> {
  for (kind, lines) in [("item", &order.items), ("item box", &order.item_boxes)] {
    let mut seen = HashSet::new();
    for line in lines {
      if line.quantity < 1 {
        return Err(format!(
          "The quantity of {kind} {} must be 1 or more",
          line.id
        ));
      }
      if !seen.insert(line.id) {
        return Err(format!(
          "The {kind} {} is on the order more than once",
          line.id
        ));
      }
    }
  }
  Ok(())
}

fn err_wrap<T, F>(cb: F) -> Result<T, String>
where
  F: FnOnce() -> Result<T, Error>,
{
  cb().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  /// An in-memory database with the real schema and test data.
  fn seeded() -> Connection {
    let connection = Connection::open_in_memory().unwrap();
    connection
      .execute_batch(include_str!("../../../../../db/soteria-schema.sql"))
      .unwrap();
    connection
      .execute_batch(include_str!("../../../../../db/data_test.sql"))
      .unwrap();
    connection
  }

  fn read(connection: &Connection, id: i64) -> CompletePurchaseOrder {
    let order = connection
      .query_row(
        "SELECT * FROM purchase_order WHERE id = ?1",
        (id,),
        PurchaseOrder::from_row,
      )
      .unwrap();
    with_lines(connection, order).unwrap()
  }

  #[test]
  fn reads_item_and_item_box_lines() {
    let connection = seeded();
    let order = read(&connection, 1);

    let items: Vec<_> = order
      .items
      .iter()
      .map(|l| (l.item.id, l.quantity))
      .collect();
    let boxes: Vec<_> = order
      .item_boxes
      .iter()
      .map(|l| (l.item_box.id, l.quantity))
      .collect();
    // Sorted by name: "Ceramic Brake Pad Set" before "Front Brake Rotor".
    assert_eq!(items, vec![(1, 12), (6, 4)]);
    assert_eq!(boxes, vec![(1, 2)]);
  }

  #[test]
  fn update_replaces_lines() {
    let mut connection = seeded();
    let current = read(&connection, 5);
    let input = CompletePurchaseOrderInput {
      purchase_order: current.purchase_order,
      items: vec![PurchaseOrderLineInput {
        id: 10,
        quantity: 7,
      }],
      item_boxes: vec![],
    };

    let transaction = connection.transaction().unwrap();
    assert_eq!(input.purchase_order.update(&transaction).unwrap(), 1);
    transaction
      .execute(
        "DELETE FROM junc_purchase_order_item WHERE purchase_order_id = 5",
        (),
      )
      .unwrap();
    transaction
      .execute(
        "DELETE FROM junc_purchase_order_item_box WHERE purchase_order_id = 5",
        (),
      )
      .unwrap();
    insert_lines(&transaction, 5, &input).unwrap();
    transaction.commit().unwrap();

    let order = read(&connection, 5);
    assert_eq!(order.items.len(), 1);
    assert_eq!((order.items[0].item.id, order.items[0].quantity), (10, 7));
    assert!(order.item_boxes.is_empty());
  }

  #[test]
  fn rejects_bad_lines() {
    let line = |id, quantity| PurchaseOrderLineInput { id, quantity };
    let input = |items| CompletePurchaseOrderInput {
      purchase_order: PurchaseOrder::default(),
      items,
      item_boxes: vec![],
    };
    assert!(validate_lines(&input(vec![line(1, 0)])).is_err());
    assert!(validate_lines(&input(vec![line(1, 1), line(1, 2)])).is_err());
    assert!(validate_lines(&input(vec![line(1, 1), line(2, 2)])).is_ok());
  }
}
