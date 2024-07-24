use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Server error struct.
///
/// Default HTTP status is `500 Internal Server Error`.
#[derive(Debug, Clone)]
pub struct ServerError {
    error: String,
    status: StatusCode,
}

impl ServerError {
    /// Create `ServerError` with any error and `StatusCode`.
    pub fn new<E>(error: E, status_code: StatusCode) -> Self
    where
        E: std::fmt::Display,
    {
        Self {
            error: error.to_string(),
            status: status_code,
        }
    }

    /// Create `ServerError` with status `500 Internal Server Error`.
    #[allow(non_snake_case)]
    pub fn INTERNAL_SERVER_ERROR<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Create `ServerError` with status `400 Bad Request`.
    #[allow(non_snake_case)]
    pub fn BAD_REQUEST<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::BAD_REQUEST)
    }

    /// Create `ServerError` with status `401 Unauthorized`.
    #[allow(non_snake_case)]
    pub fn UNAUTHORIZED<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::UNAUTHORIZED)
    }

    /// Create `ServerError` with status `403 Forbidden`.
    #[allow(non_snake_case)]
    pub fn FORBIDDEN<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::FORBIDDEN)
    }

    /// Create `ServerError` with status `403 Forbidden`.
    #[allow(non_snake_case)]
    pub fn NOT_FOUND<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::NOT_FOUND)
    }

    /// Create `ServerError` with status `409 Conflict`.
    #[allow(non_snake_case)]
    pub fn CONFLICT<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::CONFLICT)
    }

    /// Create `ServerError` with status `511 Network Authentication Required`.
    #[allow(non_snake_case)]
    pub fn NETWORK_AUTHENTICATION_REQUIRED<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (self.status, format!("Something went wrong: {}", self.error)).into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self::new(error, StatusCode::INTERNAL_SERVER_ERROR)
    }
}
