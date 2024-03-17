#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuFeatures {
    #[serde(rename = "amx", skip_serializing_if = "Option::is_none")]
    pub amx: Option<bool>,
}

impl CpuFeatures {
    pub fn new() -> CpuFeatures {
        CpuFeatures { amx: None }
    }
}
