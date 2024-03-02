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
pub struct RngConfig {
    #[serde(rename = "src")]
    pub src: String,
    #[serde(rename = "iommu", skip_serializing_if = "Option::is_none")]
    pub iommu: Option<bool>,
}

impl RngConfig {
    pub fn new(src: String) -> RngConfig {
        RngConfig {
            src,
            iommu: None,
        }
    }
}


