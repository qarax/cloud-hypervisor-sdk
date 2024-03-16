use std::{borrow::Cow, path::PathBuf};

use cloud_hypervisor_sdk::{
    machine::{Machine, MachineConfig},
    models::{memory_config::MemoryConfig, CpusConfig, DiskConfig, PayloadConfig, VmConfig},
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpus_config = CpusConfig {
        boot_vcpus: 2,
        max_vcpus: 2,
        ..Default::default()
    };

    // Size in gb
    let memory_config = MemoryConfig::new(2);

    let vm_config = VmConfig {
        cpus: Some(Box::new(cpus_config)),
        memory: Some(Box::new(memory_config)),
        payload: Box::new(PayloadConfig {
            kernel: Some("./hypervisor-fw".to_string()),
            ..Default::default()
        }),
        disks: Some(vec![DiskConfig::new("./fedora.raw".to_string())]),
        ..Default::default()
    };

    let machine_config = MachineConfig {
        vm_config,
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("./cloud-hypervisor.sock")),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    Machine::create(machine_config).await?;

    Ok(())
}
