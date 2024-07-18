use crate::error::ServerError;

pub async fn root() -> Result<(), ServerError> {
    Ok(())
}
