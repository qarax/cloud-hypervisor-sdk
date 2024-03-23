use std::{borrow::Cow, path::PathBuf};

use cloud_hypervisor_sdk::machine::{Machine, MachineConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machine_config = MachineConfig {
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("/tmp/cloud-hypervisor.sock")),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    let mut vm = Machine::connect(machine_config).await?;
    let mut vm_info = vm.get_info().await?;
    println!("{:?}", vm_info.state);
    vm.shutdown().await?;
    vm_info = vm.get_info().await?;
    println!("{:?}", vm_info.state);

    Ok(())
}
