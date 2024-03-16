use std::{borrow::Cow, path::Path, process::Stdio};

use crate::{
    client::{HttpClient, TokioIo},
    error::Error,
    models::VmConfig,
};
use tokio::net::UnixStream;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MachineConfig<'m> {
    pub vm_config: VmConfig,
    pub vm_id: Uuid,
    pub socket_path: Cow<'m, Path>,
    pub exec_path: Cow<'m, Path>,
}

#[derive(Debug)]
pub struct Machine<'m> {
    config: MachineConfig<'m>,
    client: Option<HttpClient>,
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
        let tmux_cmd = self.generate_cloud_hypervisor_cmd(&self.config.vm_config);
        println!("starting VM with tmux command: {}", tmux_cmd);
    
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
            ).into());
        }
    
        println!("Cloud Hypervisor started successfully in a tmux session, you can attach to it using:\n\n tmux attach -t vm_{}\n", self.config.vm_id);
        Ok(())
    }
    
    fn generate_cloud_hypervisor_cmd(&self, vm_config: &VmConfig) -> String {
        let mut ch_cmd = format!(
            "{} --api-socket {}",
            self.config
                .exec_path
                .to_str()
                .expect("exec_path is not a valid UTF-8 string"),
            self.config.socket_path.to_string_lossy()
        );

        if let Some(kernel_path) = &vm_config.payload.kernel {
            ch_cmd += &format!(" --kernel {}", kernel_path);
        }
        if let Some(cpus) = &vm_config.cpus {
            ch_cmd += &format!(" --cpus boot={}", cpus.boot_vcpus);
        }
        if let Some(memory) = &vm_config.memory {
            ch_cmd += &format!(" --memory size={}G", memory.size);
        }
        if let Some(initramfs_path) = &vm_config.payload.initramfs {
            ch_cmd += &format!(" --initramfs {}", initramfs_path);
        }
        if let Some(disks) = &vm_config.disks {
            for disk in disks {
                ch_cmd += &format!(" --disk path={}", disk.path);
            }
        }

        let session_name = format!("vm_{}", self.config.vm_id);
        let tmux_cmd = format!("tmux new-session -d -s '{}' {}", session_name, ch_cmd);

        tmux_cmd
    }

    async fn build_client(socket_path: &Path) -> Result<HttpClient, Error> {
        println!("connecting HTTP clinet to Cloud Hypervisor at {:?}...", socket_path);
        let stream = UnixStream::connect(socket_path).await?;
        println!("HTTP client connected to Cloud Hypervisor at {:?}", socket_path);
        let io = TokioIo::new(stream);
        let (sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // TODO: handle error
        tokio::spawn(conn);

        Ok(HttpClient::new(sender))
    }
}
