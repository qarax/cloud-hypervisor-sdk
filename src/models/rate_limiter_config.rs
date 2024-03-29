/*
 * Cloud Hypervisor API
 *
 * Local HTTP based API for managing and inspecting a cloud-hypervisor virtual machine.
 *
 * The version of the OpenAPI document: 0.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RateLimiterConfig : Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by configuring each of the _bandwidth_ and _ops_ token buckets.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateLimiterConfig {
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Box<crate::models::TokenBucket>>,
    #[serde(rename = "ops", skip_serializing_if = "Option::is_none")]
    pub ops: Option<Box<crate::models::TokenBucket>>,
}

impl RateLimiterConfig {
    /// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by configuring each of the _bandwidth_ and _ops_ token buckets.
    pub fn new() -> RateLimiterConfig {
        RateLimiterConfig {
            bandwidth: None,
            ops: None,
        }
    }
}
