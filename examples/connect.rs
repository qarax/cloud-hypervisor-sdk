use std::{borrow::Cow, path::PathBuf};

use cloud_hypervisor_sdk::machine::{Machine, MachineConfig};
use uuid::Uuid;

// TODO make this receive the path to the socket as an argument

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machine_config = MachineConfig {
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("/tmp/cloud-hypervisor.sock")),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    let mut machine = Machine::connect(machine_config).await?;
    let vm_info = machine.get_vm_info().await?;
    println!("{:?}", vm_info.state);

    Ok(())
}
