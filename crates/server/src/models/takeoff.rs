use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct Takeoff {
    /// Incrementing ID.
    pub id: i32, // serial
    /// Description.
    pub description: String, // varchar
    // /// Latitude coordinate.
    pub latitude: f64, // double precision
    /// Longitude coordinate.
    pub longitude: f64, // double precision
    /// Creation date, without timezone, as UNIX timestamp.
    pub creation_date: i64, // bigint
}
