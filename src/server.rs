use std::future::Future;
use tracing::info;

use tokio::{net::TcpListener, sync::broadcast};

use crate::client::Client;

pub async fn run_local_server(listener: TcpListener, shutdown: impl Future) -> crate::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    let mut client = Client::new(32, notify_shutdown, "127.0.0.1:2386").await?;
    client.stop().await?;
    Ok(())
}
