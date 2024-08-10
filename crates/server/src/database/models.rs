use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Generic data struct.
///
/// Can hold any data and an optional session.
#[derive(Deserialize, Serialize)]
pub struct Data<T> {
    /// Any model or value.
    pub value: T,
    /// Session (if necessary).
    pub session: Option<Session>,
}

/// Takeoff model.
///
/// * Use [`NewTakeoff`] for creating a new takeoff.
/// * Use [`GetTakeoff`] for optional fields.
#[derive(Debug, Serialize, FromRow)]
pub struct Takeoff {
    /// Incrementing ID.
    pub id: i32,
    /// String.
    pub name: String,
    /// Description.
    pub description: String,
    /// Optional image.
    pub image: Option<Vec<u8>>,
    /// Region.
    pub region: String,
    /// Optional meters over sea level
    pub altitude: Option<i32>,
    /// Optional difference between takeoff and landing in altitude.
    pub altitude_diff: Option<i32>,
    /// Latitude coordinate.
    pub latitude: f64,
    /// Longitude coordinate.
    pub longitude: f64,
    /// Wind directions.
    pub wind_dirs: Vec<String>,
    /// Optional info URL.
    pub info_url: Option<String>,
    /// Optional source URL.
    pub source_url: Option<String>,
    /// Creation date and author name.
    pub created: String,
    /// Last update date and author name.
    pub updated: String,
}

/// New takeoff model.
///
/// Used for creating new takeoffs.
#[derive(Debug, Deserialize)]
pub struct NewTakeoff {
    /// String.
    pub name: String,
    /// Description.
    pub description: String,
    /// Optional image.
    pub image: Option<Vec<u8>>,
    /// Region.
    pub region: String,
    // Optional meters over sea level
    pub altitude: Option<i32>,
    /// Optional difference between takeoff and landing in altitude.
    pub altitude_diff: Option<i32>,
    /// Latitude coordinate.
    pub latitude: f64,
    /// Longitude coordinate.
    pub longitude: f64,
    /// Wind directions.
    pub wind_dirs: Vec<String>,
    /// Optional info URL.
    pub info_url: Option<String>,
    /// Optional source URL.
    pub source_url: Option<String>,
    /// Creation date and author name.
    pub created: String,
    /// Last update date and author name.
    pub updated: String,
}

/// Get takeoff model.
///
/// Used for getting takeoffs.
#[derive(Debug, Default, Serialize, FromRow)]
#[sqlx(default)]
pub struct GetTakeoff {
    /// Incrementing ID.
    pub id: Option<i32>,
    /// String.
    pub name: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Optional image.
    pub image: Option<Vec<u8>>,
    /// Region.
    pub region: Option<String>,
    // Optional meters over sea level
    pub altitude: Option<i32>,
    /// Optional difference between takeoff and landing in altitude.
    pub altitude_diff: Option<i32>,
    /// Latitude coordinate.
    pub latitude: Option<f64>,
    /// Longitude coordinate.
    pub longitude: Option<f64>,
    /// Wind directions.
    pub wind_dirs: Option<Vec<String>>,
    /// Optional info URL.
    pub info_url: Option<String>,
    /// Optional source URL.
    pub source_url: Option<String>,
    /// Creation date and author name.
    pub created: Option<String>,
    /// Last update date and author name.
    pub updated: Option<String>,
}

/// User model.
///
/// * Use [`NewUser`] for creating a user.
/// * Use [`LoginUser`] for logging into an user.
#[derive(Deserialize, Serialize)]
pub struct User {
    /// Unique user id.
    pub id: i32,
    /// Username.
    pub username: String,
    /// Hashed and randomly salted password.
    pub password: String,
}

/// New user model.
///
/// Used for creating new users.
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    /// Username.
    pub username: String,
    /// Raw password.
    pub password: String,
}

/// Login user model.
///
/// Used to login to a user.
#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    /// Username.
    pub username: String,
    /// Raw password.
    pub password: String,
}

/// Session model.
///
/// This is the session saved by the user.
///
/// * Use [`HashedSession`] to save a session on the server.
#[derive(Deserialize, Serialize)]
pub struct Session {
    /// Unique session id.
    pub id: i32,
    /// User id.
    pub user_id: i32,
    /// Raw session token.
    pub token: Vec<u8>,
}

/// Hashed session model.
///
/// This is the session saved by the server.
///
/// * Use [`Session`] to save a session on the user.
#[derive(Deserialize, Serialize)]
pub struct HashedSession {
    /// Unique session id.
    pub id: i32,
    /// User id.
    pub user_id: i32,
    /// Hashed and randomly salted session token.
    pub token: String,
}

/// Role model.
///
/// Use this model for fetching roles.
#[derive(Deserialize, Serialize)]
pub struct Role {
    /// Unique role id.
    pub id: i32,
    /// Role name.
    pub name: String,
}
