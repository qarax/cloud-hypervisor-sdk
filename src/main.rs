use std::{borrow::Cow, path::PathBuf};

use cloud_hypervisor_sdk::{machine::{Machine, MachineConfig}, models::{memory_config, CpusConfig, DiskConfig, PayloadConfig, VmConfig}};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpus_config = CpusConfig {
        boot_vcpus: 2,
        max_vcpus: 2,
        ..Default::default()
    };

    let memory_config = memory_config::MemoryConfig {
        size: 2,
        hotplug_size: None,
        hotplugged_size: None,
        mergeable: None,
        hotplug_method: None,
        shared: None,
        hugepages: None,
        hugepage_size: None,
        prefault: None,
        thp: None,
        zones: None,
    };

    let vm_config = VmConfig {
        cpus: Some(Box::new(cpus_config)),
        memory: Some(Box::new(memory_config)),
        payload: Box::new(PayloadConfig {
            kernel: Some("./hypervisor-fw".to_string()),
            initramfs: None,
            firmware: None,
            cmdline: None,
        }),
        disks: Some(vec![DiskConfig {
            path: "fedora.raw".to_string(),
            affinity: None,
            direct: None,
            iommu: None,
            readonly: None,
            vhost_user: None,
            id: None,
            rate_limit_group: None,
            serial: None,
            vhost_socket: None,
            num_queues: None,
            queue_size: None,
            pci_segment: None,
            rate_limiter_config: None,
        }]),
        balloon: None,
        fs: None,
        rate_limit_groups: None,
        net: None,
        rng: None,
        pmem: None,
        serial: None,
        console: None,
        debug_console: None,
        devices: None,
        vdpa: None,
        vsock: None,
        sgx_epc: None,
        numa: None,
        iommu: None,
        watchdog: None,
        platform: None,
        tpm: None,
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