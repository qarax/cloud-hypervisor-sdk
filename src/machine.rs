use std::{borrow::Cow, path::Path};

use crate::{
    client::{HttpClient, TokioIo},
    error::Error,
    models::VmConfig,
};
use bytes::Bytes;
use http_body_util::{combinators::BoxBody, Full};
use hyper::Request;
use tokio::net::UnixStream;
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
    pid: Option<u32>,
}

impl<'m> Machine<'m> {
    pub async fn create(config: MachineConfig<'m>) -> Result<Machine<'m>, Error> {
        // Remove existing socket file
        if config.socket_path.exists() {
            std::fs::remove_file(&config.socket_path)?;
        }

        let mut machine = Machine {
            config: config.clone(),
            client: None,
            pid: None,
        };
        Self::start(&machine).await?;

        let client = Self::build_client(&config.socket_path).await?;
        machine.client = Some(client);

        Ok(machine)
    }

    async fn start(&self) -> Result<(), Error> {
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

        println!("Cloud Hypervisor started successfully in a tmux session, you can attach to it using:\n\n tmux attach -t vm_{}\n", self.config.vm_id);
        Ok(())
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
        println!(
            "connecting HTTP clinet to Cloud Hypervisor at {:?}...",
            socket_path
        );
        let stream = UnixStream::connect(socket_path).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // TODO: handle error
        tokio::spawn(conn);

        sender.ready().await?;

        println!(
            "HTTP client connected to Cloud Hypervisor at {:?}",
            socket_path
        );

        Ok(HttpClient::new(sender))
    }

    pub async fn create_vm(&mut self, vm_config: &VmConfig) -> Result<(), Error> {
        let client = self.client.as_mut().unwrap();
        let sender = client.sender();
        let json_vm_config = serde_json::to_vec(vm_config).unwrap();
        let body = Bytes::from(json_vm_config);

        let request = Request::builder()
            .method("PUT")
            .uri(format!("http://localhost/api/v1/vm.create"))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(BoxBody::new(Full::new(body.clone())))
            .unwrap();

        // Pretty print the request
        println!("{:#?}", request);

        // Show context of body
        println!("{:?}", body.clone());

        sender.send_request(request).await?;
        Ok(())
    }
}
