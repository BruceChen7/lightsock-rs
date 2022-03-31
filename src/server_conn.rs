use bytes::{Buf, BytesMut};
use tokio::io::BufWriter;
use tokio::net::TcpStream;

pub struct ServerConnection {
    // stream for
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
}

impl ServerConnection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> ServerConnection {
        ServerConnection {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
}
