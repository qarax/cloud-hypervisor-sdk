use thiserror::Error;
use hyper::Error as HyperError;
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum Error{
    #[error("failed to connect to Unix socket: {0}")]
    UnixSocketConnectError(#[from] IoError),
    #[error("HTTP handshake error: {0}")]
    HttpHandshakeError(#[from] HyperError),
}