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
pub struct DeviceConfig {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "iommu", skip_serializing_if = "Option::is_none")]
    pub iommu: Option<bool>,
    #[serde(rename = "pci_segment", skip_serializing_if = "Option::is_none")]
    pub pci_segment: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl DeviceConfig {
    pub fn new(path: String) -> DeviceConfig {
        DeviceConfig {
            path,
            iommu: None,
            pci_segment: None,
            id: None,
        }
    }
}
