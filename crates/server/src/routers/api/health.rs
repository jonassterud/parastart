use super::version::Version;
use crate::error::ServerError;

pub async fn get(version: Version) -> Result<String, ServerError> {
    Ok(format!("OK\n\nVersion: {:?}", version))
}
