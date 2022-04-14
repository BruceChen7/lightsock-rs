use crate::server_conn::ServerConnection;
use std::future::Future;
use tracing::{field::debug, info};

use crate::shutdown::Shutdown;
use std::io;
use tokio::{net::TcpListener, sync::broadcast};

use crate::client::{self, Client};

pub async fn run_local_server(
    listener: TcpListener,
    client: Client,
    shutdown: impl Future,
) -> crate::Result<()> {
    let mut cli = client;
    let (notify_shutdown, _) = broadcast::channel(1);
    tokio::select! {
        _ = handle_local_server(listener,&mut cli, &notify_shutdown) => {
        }
        _ = shutdown => {
            info!("local server shutdown");
        }
    }
    drop(notify_shutdown);
    cli.stop().await?;
    Ok(())
}

pub async fn handle_local_server(
    listener: TcpListener,
    client: &mut client::Client,
    notify_shutdown: &broadcast::Sender<()>,
) -> crate::Result<()> {
    info!("accepting inbound connections");
    let mut noti_receive = notify_shutdown.subscribe();
    tokio::select! {
        _ = async {
            loop {
                let (sock, _) = listener.accept().await?;
                let shutdown = Shutdown::new(notify_shutdown.subscribe());
                let conn = ServerConnection::new(sock);
                tokio::spawn(async move { process_local_server(conn, shutdown) });
            }
            Ok::<_, io::Error>(())

        }=> {}
        _ = noti_receive.recv() => {
            info!("shutdown")
        }
    }
    Ok(())
}

async fn process_local_server(conn: ServerConnection, shutdown: Shutdown) -> crate::Result<()> {
    let mut cc = conn;
    while !shutdown.is_shutdown() {
        let _ = tokio::select! {
            res = cc.read_frame() => { }
        };
    }
    Ok(())
}
