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
use tokio::{fs, net::UnixStream};
use tracing::{debug, info};
use uuid::Uuid;

/// Configuration for a `Machine`.
/// vm_id: The UUID of the VM associated with this `Machine`.
/// socket_path: The path to the socket file used to communicate with the Cloud Hypervisor instance.
/// exec_path: The path to the Cloud Hypervisor executable.
#[derive(Clone, Debug)]
pub struct MachineConfig<'m> {
    pub vm_id: Uuid,
    pub socket_path: Cow<'m, Path>,
    pub exec_path: Cow<'m, Path>,
}

/// A handle to a Cloud Hypervisor instance.
/// This struct is used to interact with the Cloud Hypervisor instance.
/// It provides methods to start the Cloud Hypervisor instance, create VMs, and interact with VMs.
pub struct Machine<'m> {
    config: MachineConfig<'m>,
    client: HttpClient,
}

/// Represents a VM created in a Cloud Hypervisor instance managed by a `Machine`.
pub struct VM<'m> {
    machine: Machine<'m>,

    // When connecting to a machine the VmConfig may not be known
    vm_config: Option<VmConfig>,
}

impl<'m> Machine<'m> {
    /// Starts the Cloud Hypervisor VMM associated with this `Machine`.
    ///
    /// This method will launch Cloud Hypervisor in a tmux session using the configured `exec_path` and `socket_path`.
    /// It will also establish an HTTP client connection to the Cloud Hypervisor instance.
    pub async fn start(config: MachineConfig<'_>) -> Result<Machine<'_>, Error> {
        if config.socket_path.exists() {
            fs::remove_file(&config.socket_path).await?;
        }

        let tmux_cmd = Self::generate_cloud_hypervisor_cmd(&config);

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

        let client = Self::build_client(&config.socket_path).await?;

        println!("Cloud Hypervisor started successfully in a tmux session, you can attach to it using:\n\n tmux attach -t vm_{}\n", config.vm_id);
        Ok(Machine { config, client })
    }

    /// Connects to an existing Cloud Hypervisor instance.
    pub async fn connect(config: MachineConfig<'m>) -> Result<VM<'m>, Error> {
        let client = Self::build_client(&config.socket_path).await?;
        let machine = Machine { config, client };

        Ok(VM {
            machine,
            vm_config: None,
        })
    }

    /// Creates a VM in the Cloud Hypervisor instance associated with this `Machine`.
    /// The VM is created with the provided `VmConfig`.
    pub async fn create_vm(mut self, vm_config: VmConfig) -> Result<VM<'m>, Error> {
        let sender = self.client.sender();
        let json_vm_config =
            serde_json::to_vec(&vm_config).map_err(|e| Error::Other(e.to_string()))?;
        let body = Bytes::from(json_vm_config);

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.create"))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(BoxBody::new(Full::new(body.clone())))?;

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = read_response_body(response).await?;
            return Err(Error::CloudHypervisorApiError(status, body));
        }

        Ok(VM {
            machine: self,
            vm_config: Some(vm_config),
        })
    }

    fn generate_cloud_hypervisor_cmd(config: &MachineConfig) -> String {
        let ch_cmd = format!(
            "{} --api-socket {}",
            config
                .exec_path
                .to_str()
                .expect("exec_path is not a valid UTF-8 string"),
            config.socket_path.to_string_lossy()
        );

        let session_name = format!("vm_{}", config.vm_id);
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

impl VM<'_> {
    /// Boots the VM.
    pub async fn boot(&mut self) -> Result<(), Error> {
        let client = &mut self.machine.client;
        let sender = client.sender();

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.boot"))
            .header("Accept", "application/json")
            .body(BoxBody::new(Empty::new()))?;

        // read response body
        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = read_response_body(response).await?;
            return Err(Error::CloudHypervisorApiError(status, body));
        }

        Ok(())
    }

    /// Returns information about the VM, in the VmInfo struct.
    pub async fn get_info(&mut self) -> Result<VmInfo, Error> {
        let client = &mut self.machine.client;

        let sender = client.sender();

        let request = Request::builder()
            .method("GET")
            .uri(format!("http://localhost/api/v1/vm.info"))
            .header("Accept", "application/json")
            .body(BoxBody::new(Empty::new()))?;

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = read_response_body(response).await?;
            return Err(Error::CloudHypervisorApiError(status, body));
        }

        // Extract State field from response
        let body = read_response_body(response).await?;
        let vm_info: VmInfo =
            serde_json::from_str(&body).map_err(|e| Error::Other(e.to_string()))?;

        Ok(vm_info)
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        let client = &mut self.machine.client;
        let sender = client.sender();

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.shutdown"))
            .header("Accept", "application/json")
            .body(BoxBody::new(Empty::new()))?;

        let response = sender.send_request(request).await?;
        debug!("{:#?}", response);

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = read_response_body(response).await?;
            return Err(Error::CloudHypervisorApiError(status, body));
        }

        Ok(())
    }
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
