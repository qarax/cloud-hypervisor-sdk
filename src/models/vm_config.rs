#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmConfig {
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Box<crate::models::CpusConfig>>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::MemoryConfig>>,
    #[serde(rename = "payload")]
    pub payload: Box<crate::models::PayloadConfig>,
    #[serde(rename = "rate_limit_groups", skip_serializing_if = "Option::is_none")]
    pub rate_limit_groups: Option<Vec<crate::models::RateLimitGroupConfig>>,
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<crate::models::DiskConfig>>,
    #[serde(rename = "net", skip_serializing_if = "Option::is_none")]
    pub net: Option<Vec<crate::models::NetConfig>>,
    #[serde(rename = "rng", skip_serializing_if = "Option::is_none")]
    pub rng: Option<Box<crate::models::RngConfig>>,
    #[serde(rename = "balloon", skip_serializing_if = "Option::is_none")]
    pub balloon: Option<Box<crate::models::BalloonConfig>>,
    #[serde(rename = "fs", skip_serializing_if = "Option::is_none")]
    pub fs: Option<Vec<crate::models::FsConfig>>,
    #[serde(rename = "pmem", skip_serializing_if = "Option::is_none")]
    pub pmem: Option<Vec<crate::models::PmemConfig>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<Box<crate::models::ConsoleConfig>>,
    #[serde(rename = "console", skip_serializing_if = "Option::is_none")]
    pub console: Option<Box<crate::models::ConsoleConfig>>,
    #[serde(rename = "debug_console", skip_serializing_if = "Option::is_none")]
    pub debug_console: Option<Box<crate::models::DebugConsoleConfig>>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::DeviceConfig>>,
    #[serde(rename = "vdpa", skip_serializing_if = "Option::is_none")]
    pub vdpa: Option<Vec<crate::models::VdpaConfig>>,
    #[serde(rename = "vsock", skip_serializing_if = "Option::is_none")]
    pub vsock: Option<Box<crate::models::VsockConfig>>,
    #[serde(rename = "sgx_epc", skip_serializing_if = "Option::is_none")]
    pub sgx_epc: Option<Vec<crate::models::SgxEpcConfig>>,
    #[serde(rename = "numa", skip_serializing_if = "Option::is_none")]
    pub numa: Option<Vec<crate::models::NumaConfig>>,
    #[serde(rename = "iommu", skip_serializing_if = "Option::is_none")]
    pub iommu: Option<bool>,
    #[serde(rename = "watchdog", skip_serializing_if = "Option::is_none")]
    pub watchdog: Option<bool>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<crate::models::PlatformConfig>>,
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none")]
    pub tpm: Option<Box<crate::models::TpmConfig>>,
}

impl VmConfig {
    /// Virtual machine configuration
    pub fn new(payload: crate::models::PayloadConfig) -> VmConfig {
        VmConfig {
            cpus: None,
            memory: None,
            payload: Box::new(payload),
            rate_limit_groups: None,
            disks: None,
            net: None,
            rng: None,
            balloon: None,
            fs: None,
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
        }
    }
}


