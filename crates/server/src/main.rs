#[tokio::main]
async fn main() -> Result<(), server_lib::ServerError> {
    server_lib::run().await
}
