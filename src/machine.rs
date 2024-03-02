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
        println!("Starting Cloud Hypervisor");
        let cmd = self.generate_cloud_hypervisor_cmd(&self.config.vm_config);
        let mut parts: std::str::SplitWhitespace<'_> = cmd.split_whitespace();
        let exec_path = parts.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "exec_path is missing",
        ))?;
        let args = parts.collect::<Vec<&str>>();
        let mut command = std::process::Command::new(exec_path);
        command.args(args);

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let child = command.spawn()?;

        println!("Starting Cloud Hypervisor with command: {:?}", cmd);

        let output = child.wait_with_output()?;
        if !output.status.success() {
            eprintln!("Cloud Hypervisor failed to start: {:?}", output);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start Cloud Hypervisor",
            )
            .into());
        }

        println!("Cloud Hypervisor started successfully");

        Ok(())
    }

    fn generate_cloud_hypervisor_cmd(&self, vm_config: &VmConfig) -> String {
        let mut cmd = self
            .config
            .exec_path
            .to_str()
            .expect("exec_path is not a valid UTF-8 string")
            .to_string();
        cmd += &format!(
            " --api-socket {}",
            self.config.socket_path.to_string_lossy()
        );
        if let Some(kernel_path) = &vm_config.payload.kernel {
            cmd += &format!(" --kernel {}", kernel_path);
        }

        if let Some(cpus) = &vm_config.cpus {
            cmd += &format!(" --cpus boot={}", cpus.boot_vcpus);
        }

        if let Some(memory) = &vm_config.memory {
            cmd += &format!(" --memory size={}G", memory.size);
        }

        if let Some(initramfs_path) = &vm_config.payload.initramfs {
            cmd += &format!(" --initramfs {}", initramfs_path);
        }

        if let Some(disks) = &vm_config.disks {
            for disk in disks {
                cmd += &format!(" --disk path={}", disk.path);
            }
        }

        cmd
    }

    async fn build_client(socket_path: &Path) -> Result<HttpClient, Error> {
        println!("Connecting to Cloud Hypervisor at {:?}", socket_path);
        let stream = UnixStream::connect(socket_path).await?;
        println!("Connected to Cloud Hypervisor at {:?}", socket_path);
        let io = TokioIo::new(stream);
        let (sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // TODO: handle error
        tokio::spawn(conn);

        Ok(HttpClient::new(sender))
    }
}
