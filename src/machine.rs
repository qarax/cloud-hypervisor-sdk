use std::{borrow::Cow, io, path::Path};

use crate::{client::HttpClient, client::TokioIo, error::Error, models};
use tokio::net::UnixStream;
use uuid::Uuid;

pub struct MachineConfig<'m> {
    vm_config: models::VmConfig,
    vm_id: Uuid,
    socket_path: Cow<'m, Path>,
}

pub struct Machine<'m> {
    config: MachineConfig<'m>,
    client: HttpClient,
}

impl<'m> Machine<'m> {
    pub async fn create(config: MachineConfig<'m>) -> Result<Machine<'m>, Error> {
        let client = Self::build_client(&config.socket_path).await?;
        Ok(Machine { config, client })
    }

    async fn build_client(socket_path: &Path) -> Result<HttpClient, Error> {
        let stream = UnixStream::connect(socket_path).await?;
        let io = TokioIo::new(stream);
        let (sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // TODO: handle error
        tokio::spawn(conn);

        Ok(HttpClient::new(sender))
    }
}
