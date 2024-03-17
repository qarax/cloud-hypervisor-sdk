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
pub struct DebugConsoleConfig {
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "mode")]
    pub mode: Mode,
    #[serde(rename = "iobase", skip_serializing_if = "Option::is_none")]
    pub iobase: Option<i32>,
}

impl DebugConsoleConfig {
    pub fn new(mode: Mode) -> DebugConsoleConfig {
        DebugConsoleConfig {
            file: None,
            mode,
            iobase: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "Pty")]
    Pty,
    #[serde(rename = "Tty")]
    Tty,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Null")]
    Null,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Off
    }
}
