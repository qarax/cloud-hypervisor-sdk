use std::{borrow::Cow, path::Path};

use crate::{
    client::{HttpClient, TokioIo},
    error::Error,
    models::{VmConfig, VmInfo},
};
use bytes::{Bytes, BytesMut};
use futures::stream::StreamExt;
use http_body_util::{combinators::BoxBody, Empty, Full};
use hyper::{body::Incoming, Request, Response};
use tokio::net::UnixStream;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MachineConfig<'m> {
    pub vm_id: Uuid,
    pub socket_path: Cow<'m, Path>,
    pub exec_path: Cow<'m, Path>,
}

#[derive(Debug)]
pub struct Machine<'m> {
    config: MachineConfig<'m>,
    pub client: Option<HttpClient>,
}

impl<'m> Machine<'m> {
    pub async fn create(config: MachineConfig<'m>) -> Result<Machine<'m>, Error> {
        // Remove existing socket file
        if config.socket_path.exists() {
            tokio::fs::remove_file(&config.socket_path).await?;
        }

        let mut machine = Machine {
            config: config.clone(),
            client: None,
        };

        Self::start(&mut machine).await?;

        Ok(machine)
    }

    pub async fn connect(config: MachineConfig<'m>) -> Result<Machine<'m>, Error> {
        let client = Self::build_client(&config.socket_path).await?;
        Ok(Machine {
            config,
            client: Some(client),
        })
    }

    pub async fn create_vm(&mut self, vm_config: &VmConfig) -> Result<(), Error> {
        let client = match self.client.as_mut() {
            Some(client) => client,
            None => Err(Error::Other("HTTP client not initialized".to_string()))?,
        };

        let sender = client.sender();
        let json_vm_config =
            serde_json::to_vec(vm_config).map_err(|e| Error::Other(e.to_string()))?;
        let body = Bytes::from(json_vm_config);

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.create"))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(BoxBody::new(Full::new(body.clone())))?;

        // Pretty print the request
        debug!("{:#?}", request);

        // Show context of body
        debug!("{:?}", body.clone());

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        if !response.status().is_success() {
            let status = response.status();
            return match Self::read_response_body(response).await {
                Ok(body_string) => Err(Error::CloudHypervisorApiError(status.into(), body_string)),
                Err(e) => Err(Error::Other(e.to_string())),
            };
        }

        Ok(())
    }

    pub async fn boot_vm(&mut self) -> Result<(), Error> {
        let client = match self.client.as_mut() {
            Some(client) => client,
            None => Err(Error::Other("HTTP client not initialized".to_string()))?,
        };
        let sender = client.sender();

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.boot"))
            .header("Accept", "application/json")
            .body(BoxBody::new(Empty::new()))?;

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        // read response body
        if !response.status().is_success() {
            let status = response.status();

            return match Self::read_response_body(response).await {
                Ok(body_string) => Err(Error::CloudHypervisorApiError(status.into(), body_string)),
                Err(e) => Err(Error::Other(e.to_string())),
            };
        }
        Ok(())
    }

    async fn read_response_body(response: Response<Incoming>) -> Result<String, Error> {
        let mut body_bytes = http_body_util::BodyStream::new(response.into_body());
        let mut bytes = BytesMut::new();
        while let Some(chunk) = body_bytes.next().await {
            let chunk = chunk?;
            bytes.extend_from_slice(&chunk.into_data().expect("chunk is not data"));
        }

        String::from_utf8(bytes.to_vec()).map_err(|e| Error::Other(e.to_string()))
    }

    async fn start(&mut self) -> Result<(), Error> {
        let tmux_cmd = self.generate_cloud_hypervisor_cmd();

        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&tmux_cmd)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to start Cloud Hypervisor in tmux: {}", stderr);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to start Cloud Hypervisor in tmux: {}", stderr),
            )
            .into());
        }

        let client = Self::build_client(&self.config.socket_path).await?;
        self.client = Some(client);

        println!("Cloud Hypervisor started successfully in a tmux session, you can attach to it using:\n\n tmux attach -t vm_{}\n", self.config.vm_id);
        Ok(())
    }

    pub async fn get_vm_info(&mut self) -> Result<VmInfo, Error> {
        let client = self
            .client
            .as_mut()
            .ok_or_else(|| Error::Other("HTTP client not initialized".to_string()))?;
        let sender = client.sender();

        let request = Request::builder()
            .method("GET")
            .uri(format!("http://localhost/api/v1/vm.info"))
            .header("Accept", "application/json")
            .body(BoxBody::new(Empty::new()))?;

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        // read response body
        if !response.status().is_success() {
            let status = response.status();

            return match Self::read_response_body(response).await {
                Ok(body_string) => Err(Error::CloudHypervisorApiError(status.into(), body_string)),
                Err(e) => Err(Error::Other(e.to_string())),
            };
        }

        // Extract State field from response
        let body = Self::read_response_body(response).await?;
        let vm_info: VmInfo =
            serde_json::from_str(&body).map_err(|e| Error::Other(e.to_string()))?;

        Ok(vm_info)
    }

    fn generate_cloud_hypervisor_cmd(&self) -> String {
        let ch_cmd = format!(
            "{} --api-socket {}",
            self.config
                .exec_path
                .to_str()
                .expect("exec_path is not a valid UTF-8 string"),
            self.config.socket_path.to_string_lossy()
        );

        let session_name = format!("vm_{}", self.config.vm_id);
        let tmux_cmd = format!("tmux new-session -d -s '{}' {}", session_name, ch_cmd);

        tmux_cmd
    }

    async fn build_client(socket_path: &Path) -> Result<HttpClient, Error> {
        info!(
            "connecting HTTP clinet to Cloud Hypervisor at {:?}...",
            socket_path
        );
        let stream = UnixStream::connect(socket_path).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // TODO: handle error
        tokio::spawn(conn);

        sender.ready().await?;

        info!(
            "HTTP client connected to Cloud Hypervisor at {:?}",
            socket_path
        );

        Ok(HttpClient::new(sender))
    }
}
