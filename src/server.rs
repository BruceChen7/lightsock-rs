use crate::server_conn::ServerConnection;
use std::{future::Future, sync::Arc, time::Duration};
use tokio::{io, net::TcpStream, time};
use tracing::info;

use crate::shutdown::Shutdown;
use tokio::{net::TcpListener, sync::broadcast};

use crate::client::Client;

pub async fn run_local_server(
    listener: TcpListener,
    client: Arc<Client>,
    shutdown: impl Future,
) -> crate::Result<()> {
    let cli = client.clone();
    // 外部shutdown信号
    // 用来退出
    let (notify_shutdown, _) = broadcast::channel(1);
    tokio::select! {
        _ = handle_local_server(listener, client, &notify_shutdown) => {
        }
        // 用来退出
        _ = shutdown => {
            info!("local server shutdown");
        }
    }
    // 通知各个connection结束任务
    drop(notify_shutdown);
    cli.stop().await?;
    Ok(())
}

pub async fn handle_accept(listener: &TcpListener) -> crate::Result<TcpStream> {
    let mut backoff = 1;

    // Try to accept a few times
    loop {
        match listener.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => {
                if backoff > 64 {
                    // Accept has failed too many times. Return the error.
                    // 返回真正的error
                    return Err(err.into());
                }
            }
        }

        // Pause execution until the back off period elapses.
        time::sleep(Duration::from_secs(backoff)).await;

        // Double the back off
        backoff *= 2;
    }
}

async fn handle_local_server(
    listener: TcpListener,
    _client: Arc<Client>,
    notify_shutdown: &broadcast::Sender<()>,
) -> crate::Result<()> {
    info!("accepting inbound connections");
    let mut noti_receive = notify_shutdown.subscribe();
    loop {
        let _res = tokio::select! {
            _ = process_local_server(&listener, notify_shutdown.subscribe()) => {}
            _ = noti_receive.recv() => {
                info!("shutdown")
            }
        };
        return Ok(());
    }
}

async fn process_local_server(
    listener: &TcpListener,
    notify_shutdown: broadcast::Receiver<()>,
) -> crate::Result<()> {
    let sock = handle_accept(listener).await?;
    let mut cc = ServerConnection::new(sock);
    let shutdown = Shutdown::new(notify_shutdown);
    // 每一个线程处理一个请求
    // TODO(ming.chen): 可以选择固定的线程池来处理?
    tokio::spawn(async move {
        while !shutdown.is_shutdown() {
            let _ = tokio::select! {
                _res = cc.read_frame() => { }
            };
        }
        Ok::<_, io::Error>(())
    });
    Ok(())
}
