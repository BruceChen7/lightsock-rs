use std::sync::Mutex;

use crate::shutdown::Shutdown;
use bytes::{Bytes, BytesMut};
use tokio::{
    io::BufWriter,
    net::TcpStream,
    sync::{broadcast, mpsc, oneshot},
};

type Response<T> = oneshot::Sender<crate::Result<T>>;

pub struct ClientPoolConnection {
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
    job_receiver: async_channel::Receiver<ReqTask>,
    shutdown: Shutdown,
}

impl ClientPoolConnection {
    pub fn new(
        socket: TcpStream,
        job_receiver: async_channel::Receiver<ReqTask>,
        notify: broadcast::Receiver<()>,
    ) -> ClientPoolConnection {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            shutdown: Shutdown::new(notify),
            job_receiver,
        }
    }

    pub async fn start(&mut self) -> crate::Result<()> {
        while !self.shutdown.is_shutdown() {
            let _ = tokio::select! {
                _res  = self.job_receiver.recv() => {

                },
                _ = self.shutdown.recv() => {
                },
            };
        }
        Ok(())
    }
}

pub struct Client {
    // connection_pools: Arc<ConnectionPool>,
    notify_shutdown: broadcast::Sender<()>,
    job_signal: async_channel::Sender<ReqTask>,
    shutdown_sender: broadcast::Sender<()>,
}

impl Client {
    pub async fn new(s: i32, addr: &str) -> crate::Result<Self> {
        let (job_sender, job_receive) = async_channel::unbounded();
        let (shutdown_sender, _) = broadcast::channel(1);
        let (notify_shutdown, _) = broadcast::channel(1);

        for _ in 0..s {
            let socket = TcpStream::connect(addr).await?;
            let mut connection =
                ClientPoolConnection::new(socket, job_receive.clone(), shutdown_sender.subscribe());
            // 多个线程来执行请求任务
            tokio::spawn(async move { connection.start().await });
        }

        Ok(Self {
            job_signal: job_sender,
            shutdown_sender,
            notify_shutdown,
        })
    }

    pub async fn request(&self, content: Bytes) -> crate::Result<Option<Bytes>> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let task = ReqTask {
            req: Some(content),
            rsp: resp_tx,
        };
        // 添加任务，返回响应
        let _ = self.add_task(task).await;
        resp_rx.await?
    }

    pub async fn stop(&self) -> crate::Result<()> {
        let _ = self.notify_shutdown.send(());
        self.shutdown().await?;
        Ok(())
    }

    pub async fn add_task(&self, task: ReqTask) -> crate::Result<bool> {
        self.job_signal.send(task).await?;
        Ok(true)
    }

    async fn shutdown(&self) -> crate::Result<()> {
        // TODO(ming.chen): waiting all client to be shutdown
        self.shutdown_sender.send(())?;
        Ok(())
    }
}

pub struct ReqTask {
    req: Option<Bytes>,
    rsp: Response<Option<Bytes>>,
}
