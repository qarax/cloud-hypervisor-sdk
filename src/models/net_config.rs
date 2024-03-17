#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetConfig {
    #[serde(rename = "tap", skip_serializing_if = "Option::is_none")]
    pub tap: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "mask", skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "host_mac", skip_serializing_if = "Option::is_none")]
    pub host_mac: Option<String>,
    #[serde(rename = "mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "iommu", skip_serializing_if = "Option::is_none")]
    pub iommu: Option<bool>,
    #[serde(rename = "num_queues", skip_serializing_if = "Option::is_none")]
    pub num_queues: Option<i32>,
    #[serde(rename = "queue_size", skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<i32>,
    #[serde(rename = "vhost_user", skip_serializing_if = "Option::is_none")]
    pub vhost_user: Option<bool>,
    #[serde(rename = "vhost_socket", skip_serializing_if = "Option::is_none")]
    pub vhost_socket: Option<String>,
    #[serde(rename = "vhost_mode", skip_serializing_if = "Option::is_none")]
    pub vhost_mode: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "pci_segment", skip_serializing_if = "Option::is_none")]
    pub pci_segment: Option<i32>,
    #[serde(
        rename = "rate_limiter_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limiter_config: Option<Box<crate::models::RateLimiterConfig>>,
}

impl NetConfig {
    pub fn new() -> NetConfig {
        NetConfig {
            ..Default::default()
        }
    }
}
