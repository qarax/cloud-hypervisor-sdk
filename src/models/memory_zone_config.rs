#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryZoneConfig {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "mergeable", skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "hugepages", skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<bool>,
    #[serde(rename = "hugepage_size", skip_serializing_if = "Option::is_none")]
    pub hugepage_size: Option<i64>,
    #[serde(rename = "host_numa_node", skip_serializing_if = "Option::is_none")]
    pub host_numa_node: Option<i32>,
    #[serde(rename = "hotplug_size", skip_serializing_if = "Option::is_none")]
    pub hotplug_size: Option<i64>,
    #[serde(rename = "hotplugged_size", skip_serializing_if = "Option::is_none")]
    pub hotplugged_size: Option<i64>,
    #[serde(rename = "prefault", skip_serializing_if = "Option::is_none")]
    pub prefault: Option<bool>,
}

impl MemoryZoneConfig {
    pub fn new(id: String, size: i64) -> MemoryZoneConfig {
        MemoryZoneConfig {
            id,
            size,
            file: None,
            mergeable: None,
            shared: None,
            hugepages: None,
            hugepage_size: None,
            host_numa_node: None,
            hotplug_size: None,
            hotplugged_size: None,
            prefault: None,
        }
    }
}


