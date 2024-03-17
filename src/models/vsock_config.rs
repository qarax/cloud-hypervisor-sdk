/*
 * Cloud Hypervisor API
 *
 * Local HTTP based API for managing and inspecting a cloud-hypervisor virtual machine.
 *
 * The version of the OpenAPI document: 0.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VsockConfig {
    /// Guest Vsock CID
    #[serde(rename = "cid")]
    pub cid: i64,
    /// Path to UNIX domain socket, used to proxy vsock connections.
    #[serde(rename = "socket")]
    pub socket: String,
    #[serde(rename = "iommu", skip_serializing_if = "Option::is_none")]
    pub iommu: Option<bool>,
    #[serde(rename = "pci_segment", skip_serializing_if = "Option::is_none")]
    pub pci_segment: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl VsockConfig {
    pub fn new(cid: i64, socket: String) -> VsockConfig {
        VsockConfig {
            cid,
            socket,
            iommu: None,
            pci_segment: None,
            id: None,
        }
    }
}
