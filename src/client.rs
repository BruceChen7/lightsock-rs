use std::sync::Arc;

use crate::shutdown::Shutdown;
use bytes::{BufMut, Bytes, BytesMut};
use tokio::{
    io::BufWriter,
    net::TcpStream,
    sync::{broadcast, broadcast::Sender, mpsc, oneshot},
};

type Response<T> = oneshot::Sender<crate::Result<T>>;

#[derive(Debug)]
struct Connection {
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
    job_receiver: async_channel::Receiver<ReqTask>,
    shutdown: Shutdown,
    shutdown_complete: mpsc::Sender<()>,
}

impl Connection {
    pub fn new(
        socket: TcpStream,
        job_receiver: async_channel::Receiver<ReqTask>,
        notify: broadcast::Receiver<()>,
        shutdown_complete: mpsc::Sender<()>,
    ) -> Connection {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            shutdown: Shutdown::new(notify),
            job_receiver,
            shutdown_complete,
        }
    }

    pub async fn start(&self) {
        loop {
            let _ = self.job_receiver.recv().await;
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
        tokio::spawn(async move { p.add_task(task).await.unwrap() });
        resp_rx.await?
    }
}

struct ConnectionPool {
    size: i32,
    job_signal: async_channel::Sender<ReqTask>,
    shutdown_sender: broadcast::Sender<()>,
    shutdown_complete: mpsc::Receiver<()>,
}

impl ConnectionPool {
    pub async fn new(s: i32, addr: &str) -> crate::Result<ConnectionPool> {
        let (job_sender, job_receive) = async_channel::unbounded();
        let (shutdown_sender, _) = broadcast::channel(1);
        let (connection_shutdown_complete, connection_shutdown_recv) = mpsc::channel(s as usize);

        for _ in 0..s {
            let socket = TcpStream::connect(addr).await?;
            let connection = Connection::new(
                socket,
                job_receive.clone(),
                shutdown_sender.subscribe(),
                connection_shutdown_complete.clone(),
            );
            let h = tokio::spawn(async move { connection.start().await });
            h.await.unwrap()
        }
        Ok(Self {
            size: s,
            job_signal: job_sender,
            shutdown_sender,
            shutdown_complete: connection_shutdown_recv,
        })
    }

    pub async fn add_task(&self, task: ReqTask) -> crate::Result<bool> {
        self.job_signal.send(task).await?;
        Ok(true)
    }

    pub async fn shutdown(&mut self) -> crate::Result<()> {
        let mut i = 0;
        self.shutdown_sender.send(())?;
        loop {
            if self.size == i {
                return Ok(());
            }
            self.shutdown_complete.recv().await;
            i = i + 1
        }
    }
}

struct ReqTask {
    req: Option<Bytes>,
    rsp: Response<Option<Bytes>>,
}
