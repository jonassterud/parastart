use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Takeoff {
    /// Incrementing ID.
    pub id: i32, // SERIAL PRIMARY KEY
    /// Description.
    pub body: String, // VARCHAR(2048)
    /// Optional image.
    pub picture: Option<Vec<u8>>, // BYTEA
    /// Latitude coordinate.
    pub latitude: f64, // DOUBLE PRECISION
    /// Longitude coordinate.
    pub longitude: f64, // DOUBLE PRECISION
    /// Creation date, as UNIX timestamp.
    pub creation: i64, // BIGINT
}

#[derive(Debug, Deserialize)]
pub struct CreateTakeoff {
    /// Description.
    pub body: String, // VARCHAR(2048)
    /// Optional image.
    pub picture: Option<Vec<u8>>, // BYTEA
    /// Latitude coordinate.
    pub latitude: f64, // DOUBLE PRECISION
    /// Longitude coordinate.
    pub longitude: f64, // DOUBLE PRECISION
}
