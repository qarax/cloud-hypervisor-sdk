use http::Error as HttpError;
use hyper::Error as HyperError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to connect to Unix socket: {0}")]
    UnixSocketConnectError(#[from] IoError),
    #[error("hyper error: {0}")]
    HyperError(#[from] HyperError),
    #[error("HTTP client not initialized")]
    ClientNotInitialized,
    #[error("Cloud Hyperisor API error, status: {0}, error: {1}")]
    CloudHypervisorApiError(u16, String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] HttpError),
    #[error("Unexpected error: {0}")]
    Other(String),
}
