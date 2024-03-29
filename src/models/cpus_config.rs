use super::{CpuAffinity, CpuFeatures};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuTopology {
    pub threads_per_core: Option<i32>,
    pub cores_per_die: Option<i32>,
    pub dies_per_package: Option<i32>,
    pub packages: Option<i32>,
}

impl Default for CpuTopology {
    fn default() -> Self {
        Self {
            threads_per_core: Some(1),
            cores_per_die: Some(1),
            dies_per_package: Some(1),
            packages: Some(1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CpusConfig {
    pub boot_vcpus: i32,
    pub max_vcpus: i32,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub topology: Option<Box<CpuTopology>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub kvm_hyperv: Option<bool>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub max_phys_bits: Option<i32>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Vec<CpuAffinity>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Box<CpuFeatures>>,
}

impl CpusConfig {
    pub fn new(boot_vcpus: i32, max_vcpus: i32) -> CpusConfig {
        CpusConfig {
            boot_vcpus,
            max_vcpus,
            ..Default::default()
        }
    }
}
