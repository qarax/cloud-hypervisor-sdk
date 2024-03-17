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
pub struct SgxEpcConfig {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "prefault", skip_serializing_if = "Option::is_none")]
    pub prefault: Option<bool>,
}

impl SgxEpcConfig {
    pub fn new(id: String, size: i64) -> SgxEpcConfig {
        SgxEpcConfig {
            id,
            size,
            prefault: None,
        }
    }
}
