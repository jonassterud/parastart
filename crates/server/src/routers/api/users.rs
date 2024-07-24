use std::sync::{Arc, Mutex};
use crate::database::{auth, models};
use crate::error::ServerError;
use axum::routing::get;
use axum::{routing::post, Extension, Json, Router};
use rand_chacha::ChaCha8Rng;
use sqlx::PgPool;

use super::version::Version;

pub fn router() -> Router {
    Router::new()
        .route("/api/:version/users", post(post_users))
        .route("/api/:version/login", post(post_login))
        .route("/api/:version/logout", get(get_logout))
}

/// Creates a user.
async fn post_users(
    version: Version,
    pool: Extension<PgPool>,
    Json(data): Json<models::Data<models::NewUser>>,
) -> Result<(), ServerError> {
    let username = data.value.username;
    let password_hashed =
        bcrypt::hash_with_result(data.value.password, bcrypt::DEFAULT_COST)?.to_string();

    sqlx::query!(
        r#"
            INSERT INTO "users" (username, password)
            VALUES ($1, $2)
        "#,
        username,
        password_hashed
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Checks users credentials, creates and returns session.
async fn post_login(
    version: Version,
    pool: Extension<PgPool>,
    random: Extension<Arc<Mutex<ChaCha8Rng>>>,
    Json(data): Json<models::Data<models::LoginUser>>,
) -> Result<Json<models::Session>, ServerError> {
    let user_id = auth::check_credentials((&*pool).clone(), data.value).await?;
    let session = auth::create_session((&*pool).clone(), (&*random).clone(), user_id).await?;

    Ok(Json(session))
}

/// Logout.
async fn get_logout(
    version: Version,
    pool: Extension<PgPool>,
    Json(data): Json<models::Data<()>>,
) -> Result<(), ServerError> {
    todo!();

    Ok(())
}
