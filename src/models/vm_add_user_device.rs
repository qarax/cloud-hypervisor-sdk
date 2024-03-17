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
pub struct VmAddUserDevice {
    #[serde(rename = "socket")]
    pub socket: String,
}

impl VmAddUserDevice {
    pub fn new(socket: String) -> VmAddUserDevice {
        VmAddUserDevice { socket }
    }
}
