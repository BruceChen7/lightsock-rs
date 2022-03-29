use lightsock_rs::server;
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() -> lightsock_rs::Result<()> {
    tracing_subscriber::fmt::try_init()?;
    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", "6328")).await?;

    server::run_local_server(listener, signal::ctrl_c()).await;
    Ok(())
}
