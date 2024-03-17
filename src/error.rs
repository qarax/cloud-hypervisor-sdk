use hyper::Error as HyperError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to connect to Unix socket: {0}")]
    UnixSocketConnectError(#[from] IoError),
    #[error("HTTP handshake error: {0}")]
    HttpHandshakeError(#[from] HyperError),
    #[error("HTTP client not initialized: {message}")]
    ClientNotInitialized { message: String },
}
