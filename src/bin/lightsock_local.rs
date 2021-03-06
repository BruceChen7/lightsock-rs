use std::sync::Arc;

use lightsock_rs::client;
use lightsock_rs::server;
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() -> lightsock_rs::Result<()> {
    tracing_subscriber::fmt::try_init()?;
    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", "6328")).await?;
    // remote client
    let client = client::Client::new(32, "127.0.0.1:2386").await?;
    let shared_client_pool = Arc::new(client);

    server::run_server(listener, shared_client_pool, signal::ctrl_c()).await?;
    Ok(())
}
