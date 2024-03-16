#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskConfig {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub direct: Option<bool>,
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
    #[serde(rename = "rate_limiter_config", skip_serializing_if = "Option::is_none")]
    pub rate_limiter_config: Option<Box<crate::models::RateLimiterConfig>>,
    #[serde(rename = "pci_segment", skip_serializing_if = "Option::is_none")]
    pub pci_segment: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "rate_limit_group", skip_serializing_if = "Option::is_none")]
    pub rate_limit_group: Option<String>,
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Vec<crate::models::VirtQueueAffinity>>,
}

impl DiskConfig {
    pub fn new(path: String) -> DiskConfig {
        DiskConfig {
            path,
            readonly: None,
            direct: None,
            iommu: None,
            num_queues: None,
            queue_size: None,
            vhost_user: None,
            vhost_socket: None,
            rate_limiter_config: None,
            pci_segment: None,
            id: None,
            serial: None,
            rate_limit_group: None,
            affinity: None,
        }
    }
}


