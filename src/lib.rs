pub mod cipher;
mod client;
pub mod server;
mod shutdown;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
