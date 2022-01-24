use tokio::net::{TcpListener, TcpStream};

use crate::client::Client;

struct ProxyServer {
}

impl ProxyServer {
    fn new() -> Self {
        Self {
        }
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

async fn process(socket: TcpStream) {
}
