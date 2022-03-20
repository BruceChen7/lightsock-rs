use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::client::Client;

pub struct ProxyServer {
    client: Client,
}

impl ProxyServer {
    pub async fn new() -> crate::Result<ProxyServer> {
        let (notify_shutdown, _) = broadcast::channel(1);
        let client = Client::new(32, notify_shutdown, "127.0.0.1:6333").await?;
        Ok(ProxyServer { client })
    }

    async fn run(&mut self) {
        // Bind the listener to the address
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        loop {
            // The second item contains the ip and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();

            // A new task is spawned for each inbound socket.  The socket is
            // moved to the new task and processed there.
            tokio::spawn(async move {
                process(socket).await;
            });
        }
    }
}

async fn process(socket: TcpStream) {}
