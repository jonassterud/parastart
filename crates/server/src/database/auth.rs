//! Helpers for authentication.

use super::models;
use crate::error::ServerError;

use rand::RngCore;
use rand_chacha::ChaCha8Rng;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

// TODO: Make sure the session token is created correctly.
/// Create and set session token for the given `user_id`.
///
/// Returns the unhashed session.
///
/// **Remember to check credentials before using this function!**
pub async fn create_session(
    db: PgPool,
    random: Arc<Mutex<ChaCha8Rng>>,
    user_id: i32,
) -> Result<models::Session, ServerError> {
    // Create unhashed and hashed session token
    let mut u128_pool = [0u8; 16];
    random.lock().unwrap().fill_bytes(&mut u128_pool);

    let session_token = u128_pool.to_vec();
    let session_token_hashed = bcrypt::hash(u128_pool, bcrypt::DEFAULT_COST)?;

    // Insert the hashed session into database and get the session id.
    let session_id = sqlx::query!(
        r#"
            INSERT INTO "sessions" (user_id, token)
            VALUES ($1, $2)
            RETURNING id
        "#,
        user_id,
        session_token_hashed
    )
    .fetch_one(&db)
    .await?
    .id;

    // Construct the unhashed session
    let session = models::Session {
        id: session_id,
        user_id: user_id,
        token: session_token,
    };

    Ok(session)
}

/// Check user credentials.
///
/// Returns the user id.
pub async fn check_credentials(
    db: PgPool,
    login_user: models::LoginUser,
) -> Result<i32, ServerError> {
    // Find the user from database with matching username
    let found_user = sqlx::query_as!(
        models::User,
        r#"
            SELECT * FROM "users" WHERE username = $1
        "#,
        login_user.username
    )
    .fetch_one(&db)
    .await
    .or(Err(ServerError::UNAUTHORIZED("wrong username")))?;

    // Verify the credentials
    match bcrypt::verify(login_user.password, &found_user.password) {
        Ok(true) => Ok(found_user.id),
        _ => Err(ServerError::UNAUTHORIZED("wrong password")),
    }
}

/// Checks authentication of session, returning the user id.
pub async fn check_session(
    db: PgPool,
    session: &Option<models::Session>,
) -> Result<i32, ServerError> {
    // Fetch the matching session from database
    let session = session
        .as_ref()
        .ok_or(ServerError::UNAUTHORIZED("missing session"))?;
    let found_session = sqlx::query_as!(
        models::HashedSession,
        r#"
            SELECT * FROM "sessions" WHERE id = $1 AND user_id = $2
        "#,
        session.id,
        session.user_id
    )
    .fetch_one(&db)
    .await
    .or(Err(ServerError::UNAUTHORIZED("no session found")))?;

    // Verify the session
    match bcrypt::verify(&session.token, &found_session.token) {
        Ok(true) => Ok(session.user_id),
        _ => Err(ServerError::UNAUTHORIZED("wrong token")),
    }
}

/// Check if user has a certain role.
///
/// Returns `true` or `false`.
pub async fn has_role(db: PgPool, user_id: i32, role: &str) -> Result<bool, ServerError> {
    // TODO: I think this is right?
    let user_roles: Vec<models::Role> = sqlx::query_as!(
        models::Role,
        r#"
        SELECT * FROM roles WHERE id = ANY (
            SELECT role_id FROM users_roles WHERE user_id = $1
        )
        "#,
        user_id
    )
    .fetch_all(&db)
    .await?;

    let user_has_role = user_roles.iter().any(|user_role| user_role.name == role);

    Ok(user_has_role)
}
