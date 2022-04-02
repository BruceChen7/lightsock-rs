use crate::server_conn::ServerConnection;
use std::future::Future;
use tracing::info;

use crate::shutdown::Shutdown;
use tokio::{net::TcpListener, sync::broadcast};

use crate::client::{self, Client};

pub async fn run_local_server(
    listener: TcpListener,
    client: Client,
    shutdown: impl Future,
) -> crate::Result<()> {
    let mut c = client;
    tokio::select! {
        _ = handle_local_server(listener,&mut c) => {
        }
        _ = shutdown => {
            info!("local server shutdown");
        }
    }
    c.stop().await?;
    Ok(())
}

pub async fn handle_local_server(
    listener: TcpListener,
    client: &mut client::Client,
) -> crate::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    info!("accepting inbound connections");
    loop {
        let (sock, _) = listener.accept().await?;
        let shutdown = Shutdown::new(notify_shutdown.subscribe());
        let conn = ServerConnection::new(sock);
        tokio::spawn(async move { process_local_server(conn, shutdown) });
    }
}

async fn process_local_server(conn: ServerConnection, shutdown: Shutdown) -> crate::Result<()> {
    while !shutdown.is_shutdown() {
        // let maybe_frame = tokio::select! {
        //     res = &mut conn.read_frame() => res?,
        // };
    }
    Ok(())
}
