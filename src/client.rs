use std::sync::{Arc, Mutex};

use bytes::{BufMut, Bytes, BytesMut};
use tokio::{
    io::BufWriter,
    net::TcpStream,
    sync::{broadcast, broadcast::Sender, futures::Notified, mpsc, oneshot},
};

type Response<T> = oneshot::Sender<crate::Result<T>>;

#[derive(Debug)]
struct Connection {
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
}

pub struct Client {
    connection_pools: Arc<ConnectionPool>,
    notify_shutdown: broadcast::Sender<()>,
}

impl Client {
    pub async fn new(s: i32, notify: Sender<()>, addr: &str) -> crate::Result<Self> {
        let pool = ConnectionPool::new(s, addr).await?;
        Ok(Self {
            connection_pools: Arc::new(pool),
            notify_shutdown: notify,
        })
    }

    pub async fn request(&mut self, content: Bytes) -> crate::Result<Option<Bytes>> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let task = ReqTask {
            req: Some(content),
            rsp: resp_tx,
        };
        let p = self.connection_pools.clone();
        tokio::spawn(async move {
            p.add_task(task).await;
        });
        resp_rx.await?
    }
}

struct ConnectionPool {
    connections: Mutex<Vec<Connection>>,
}

impl ConnectionPool {
    pub async fn new(s: i32, addr: &str) -> crate::Result<ConnectionPool> {
        let mut connections: Vec<Connection> = vec![];
        for _ in 0..s {
            let socket = TcpStream::connect(addr).await?;
            let connection = Connection::new(socket);
            connections.push(connection);
        }
        Ok(Self {
            connections: Mutex::new(connections),
        })
    }

    pub async fn add_task(&self, task: ReqTask) -> crate::Result<bool> {
        let connections = self.connections.lock().unwrap();
        Ok(true)
    }
}

struct ReqTask {
    req: Option<Bytes>,
    rsp: Response<Option<Bytes>>,
}
