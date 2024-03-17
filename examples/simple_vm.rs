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

    let memory_config = MemoryConfig::new(1 * 1024 * 1024 * 1024);

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
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("/tmp/cloud-hypervisor.sock")),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    let mut machine = Machine::create(machine_config).await?;
    machine.create_vm(&vm_config).await?;
    machine.boot_vm().await?;

    let vm_info = machine.get_vm_info().await?;
    println!("{:?}", vm_info.state);

    Ok(())
}
