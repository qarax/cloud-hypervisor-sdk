#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuAffinity {
    #[serde(rename = "vcpu")]
    pub vcpu: i32,
    #[serde(rename = "host_cpus")]
    pub host_cpus: Vec<i32>,
}

impl CpuAffinity {
    pub fn new(vcpu: i32, host_cpus: Vec<i32>) -> CpuAffinity {
        CpuAffinity {
            vcpu,
            host_cpus,
        }
    }
}


