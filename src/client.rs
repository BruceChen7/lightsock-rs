use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::{broadcast, broadcast::Sender, futures::Notified, mpsc},
};

#[derive(Debug)]
pub struct Connection {
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
    conn_size: i32,
    connections: Vec<Connection>,
    task_recv: mpsc::Receiver<()>,
    task_sender: mpsc::Sender<()>,
    notify_shutdown: broadcast::Sender<()>,
}

pub enum ReqTask {
    Frame(String),
}

impl Client {
    pub fn new(s: i32, notify: Sender<()>) -> Self {
        let (task_tx, task_rx) = mpsc::channel(s as usize);
        Self {
            conn_size: s,
            connections: vec![],
            task_sender: task_tx,
            task_recv: task_rx,
            notify_shutdown: notify,
        }
    }
    pub async fn start(&mut self) -> crate::Result<()> {
        for _ in 0..self.conn_size {
            let socket = TcpStream::connect("127.0.0.1:3333").await?;
            let connection = Connection::new(socket);
            self.connections.push(connection);
            tokio::spawn(async move {
            });
        }
        return Ok(())
    }

    pub async fn request(task: ReqTask) {}

    fn add_task(task: ReqTask) {}
}
