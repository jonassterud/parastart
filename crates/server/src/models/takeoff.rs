use postgres_types::{FromSql, ToSql};
use geo_types::Point;
use chrono::NaiveDateTime;

#[derive(Debug, ToSql, FromSql)]
pub struct Takeoff {
    /// Incrementing ID.
    pub id: i32,
    /// Description.
    pub description: String,
    /// Latitude and Longitude position.
    pub position: Point<f64>,
    /// Creation date (without timezone).
    pub creation_date: NaiveDateTime,
}
