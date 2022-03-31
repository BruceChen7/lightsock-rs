use crate::server_conn::ServerConnection;
use std::future::Future;
use tracing::info;

use tokio::{net::TcpListener, sync::broadcast};

use crate::client::{self, Client};

pub async fn run_local_server(listener: TcpListener, shutdown: impl Future) -> crate::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    let mut client = Client::new(32, notify_shutdown, "127.0.0.1:2386").await?;
    tokio::select! {
        _ = handle_local_server(listener,&mut client) => {
        }
        _ = shutdown => {
            info!("local server shutdown");
        }
    }
    client.stop().await?;
    Ok(())
}
pub async fn handle_local_server(
    listener: TcpListener,
    client: &mut client::Client,
) -> crate::Result<()> {
    info!("accepting inbound connections");
    loop {
        let (sock, _) = listener.accept().await?;
        let conn = ServerConnection::new(sock);
        tokio::spawn(async move { process_local_server(conn) });
    }
    Ok(())
}

async fn process_local_server(conn: ServerConnection) -> crate::Result<()> {
    Ok(())
}
