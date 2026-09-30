use crate::app::database::constants::DATABASE_PATH;
use rusqlite::{CachedStatement, Connection, Error, Result};

/// An appointment.
#[allow(unused)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Appointment {
  pub id: i64,

  /// Appointment number.
  pub number: i64,

  /// Date when the appointment was taken.
  pub date_taken: String,

  /// Date when the appointment took place.
  pub date_occuring: String,

  /// Whether the appointment was approved.
  pub approved: bool,

  /// Foreign key to the a service.
  pub service_id: i64,

  /// Foreign key to a customer.
  pub customer_id: i64,
}

/// Obtain a series of appointments.
#[tauri::command]
pub(crate) fn get_appointments() -> Result<Vec<Appointment>, String> {
  err_wrap(|| {
    let connection: Connection = Connection::open(DATABASE_PATH)?;
    let mut con_stmt: CachedStatement<'_> =
      connection.prepare_cached("SELECT * FROM appointment")?;

    let collect_values: Result<Vec<Appointment>> = con_stmt
      .query_map([], |v| {
        let (id, number, date_taken, date_occuring, approved, service_id, customer_id) =
          v.try_into()?;
        Ok(Appointment {
          id,
          number,
          date_taken,
          date_occuring,
          approved,
          service_id,
          customer_id,
        })
      })?
      .collect();
    let appointments: Vec<Appointment> = collect_values?;
    drop(con_stmt);

    connection.close().map_err(|(_, v)| v)?;
    Ok::<Vec<Appointment>, Error>(appointments)
  })
}

/// Add an appointment.
#[tauri::command]
pub(crate) fn add_appointment(appointment: Appointment) -> Result<(), String> {
  err_wrap(|| {
    let connection: Connection = Connection::open(DATABASE_PATH)?;
    let Appointment {
      id: _,
      number,
      date_occuring,
      date_taken,
      approved,
      service_id,
      customer_id,
    } = appointment;

    connection.execute(
      "INSERT INTO appointment (appointment_number, date_taken, date_occurring, approved, service_id, customer_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
      (number, date_taken, date_occuring, approved, service_id, customer_id)
    )?;

    Ok(())
  })
}

#[tauri::command]
pub(crate) fn remove_appointment(id: i64) -> Result<(), String> {
  err_wrap(|| {
    let connection = Connection::open(DATABASE_PATH)?;
    connection.execute("DELETE FROM appointment WHERE id = ?1", (id,))?;
    Ok(())
  })
}

fn err_wrap<T, F>(cb: F) -> Result<T, String>
where
  F: FnOnce() -> Result<T, Error>,
{
  cb().map_err(|e| e.to_string())
}
