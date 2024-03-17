#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PayloadConfig {
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[serde(rename = "kernel", skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(rename = "cmdline", skip_serializing_if = "Option::is_none")]
    pub cmdline: Option<String>,
    #[serde(rename = "initramfs", skip_serializing_if = "Option::is_none")]
    pub initramfs: Option<String>,
}

impl PayloadConfig {
    /// Payloads to boot in guest
    pub fn new() -> PayloadConfig {
        PayloadConfig {
            ..Default::default()
        }
    }
}
