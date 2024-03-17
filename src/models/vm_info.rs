#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmInfo {
    #[serde(rename = "config")]
    pub config: Box<crate::models::VmConfig>,
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "memory_actual_size", skip_serializing_if = "Option::is_none")]
    pub memory_actual_size: Option<i64>,
    #[serde(rename = "device_tree", skip_serializing_if = "Option::is_none")]
    pub device_tree: Option<::std::collections::HashMap<String, crate::models::DeviceNode>>,
}

impl VmInfo {
    /// Virtual Machine information
    pub fn new(config: crate::models::VmConfig, state: State) -> VmInfo {
        VmInfo {
            config: Box::new(config),
            state,
            memory_actual_size: None,
            device_tree: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Shutdown")]
    Shutdown,
    #[serde(rename = "Paused")]
    Paused,
}

impl Default for State {
    fn default() -> State {
        Self::Created
    }
}
