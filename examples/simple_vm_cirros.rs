use std::{borrow::Cow, path::PathBuf};

use cloud_hypervisor_sdk::{
    machine::{Machine, MachineConfig},
    models::{
        ConsoleConfig, CpusConfig, DiskConfig, PayloadConfig, VmConfig, memory_config::MemoryConfig,
    },
};
use uuid::Uuid;

/// Example of creating a simple VM with Cirros Linux
///
/// Before running this example, fetch the test images:
///   ./examples/fetch_test_images.sh
///
/// This will download:
/// - hypervisor-fw (UEFI firmware)
/// - Cirros cloud image (~15MB, minimal test OS)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if images exist
    let image_dir = PathBuf::from("examples/images");
    let fw_path = image_dir.join("hypervisor-fw");
    let disk_path = image_dir.join("cirros.raw");

    if !fw_path.exists() || !disk_path.exists() {
        eprintln!("Error: Required images not found!");
        eprintln!();
        eprintln!("Please run the following command to download test images:");
        eprintln!("  ./examples/fetch_test_images.sh");
        eprintln!();
        eprintln!("Or manually place the following files:");
        eprintln!("  - examples/images/hypervisor-fw");
        eprintln!("  - examples/images/cirros.raw");
        return Err("Missing required VM images".into());
    }

    // Configure VM with minimal resources
    let cpus_config = CpusConfig {
        boot_vcpus: 1,
        max_vcpus: 1,
        ..Default::default()
    };

    // 512MB RAM
    let memory_config = MemoryConfig::new(512 * 1024 * 1024);

    // Configure serial console to output to the tmux terminal
    let serial_config = ConsoleConfig {
        mode: cloud_hypervisor_sdk::models::console_config::Mode::Tty,
        file: None,
        socket: None,
        iommu: None,
    };

    let vm_config = VmConfig {
        cpus: Some(Box::new(cpus_config)),
        memory: Some(Box::new(memory_config)),
        payload: Box::new(PayloadConfig {
            kernel: Some(fw_path.to_string_lossy().to_string()),
            ..Default::default()
        }),
        disks: Some(vec![DiskConfig {
            path: Some(disk_path.to_string_lossy().to_string()),
            ..Default::default()
        }]),
        serial: Some(Box::new(serial_config)),
        ..Default::default()
    };

    let socket_path = format!("/tmp/cloud-hypervisor-{}.sock", Uuid::new_v4());

    let machine_config = MachineConfig {
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from(&socket_path)),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    println!("Starting Cloud Hypervisor VM with Cirros Linux...");
    let machine = Machine::start(machine_config).await?;

    println!("Creating VM...");
    let mut vm = machine.create_vm(vm_config).await?;

    println!("Booting VM...");
    vm.boot().await?;

    println!("Getting VM info...");
    let vm_info = vm.get_info().await?;
    println!("VM State: {:?}", vm_info.state);

    println!();
    println!("VM is running! You can:");
    println!("  - View VM console by attaching to the tmux session (see output above)");
    println!("  - Shut down with: cargo run --example shutdown_vm");
    println!();
    println!("Note: This example doesn't shut down the VM automatically.");
    println!("      The VM will keep running until you manually shut it down.");
    println!();
    println!("Cirros login credentials:");
    println!("  username: cirros");
    println!("  password: gocubsgo");

    Ok(())
}
