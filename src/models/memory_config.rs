/*
 * Cloud Hypervisor API
 *
 * Local HTTP based API for managing and inspecting a cloud-hypervisor virtual machine.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "hotplug_size", skip_serializing_if = "Option::is_none")]
    pub hotplug_size: Option<i64>,
    #[serde(rename = "hotplugged_size", skip_serializing_if = "Option::is_none")]
    pub hotplugged_size: Option<i64>,
    #[serde(rename = "mergeable", skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(rename = "hotplug_method", skip_serializing_if = "Option::is_none")]
    pub hotplug_method: Option<String>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "hugepages", skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<bool>,
    #[serde(rename = "hugepage_size", skip_serializing_if = "Option::is_none")]
    pub hugepage_size: Option<i64>,
    #[serde(rename = "prefault", skip_serializing_if = "Option::is_none")]
    pub prefault: Option<bool>,
    #[serde(rename = "thp", skip_serializing_if = "Option::is_none")]
    pub thp: Option<bool>,
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<models::MemoryZoneConfig>>,
}

impl MemoryConfig {
    pub fn new(size: i64) -> MemoryConfig {
        MemoryConfig {
            size,
            hotplug_size: None,
            hotplugged_size: None,
            mergeable: None,
            hotplug_method: None,
            shared: None,
            hugepages: None,
            hugepage_size: None,
            prefault: None,
            thp: None,
            zones: None,
        }
    }
}

