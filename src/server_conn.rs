use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ServerConnection {
    // stream for writing
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
}

#[derive(Debug, Clone)]
pub enum Frame {
    Simple(Bytes),
}

impl ServerConnection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> ServerConnection {
        ServerConnection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
    pub async fn read_frame(&mut self) -> crate::Result<Bytes> {
        loop {
            // 一个完整的包
            let _frame = self.parse_frame()?;

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok("".into());
                } else {
                    return Ok("".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Frame> {
        Ok(Frame::Simple(Bytes::from("hello world")))
    }
}
