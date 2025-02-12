/*
 * Nomad
 *
 * Nomad OpenApi specification
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreemptionConfig {
    #[serde(rename = "SystemSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub system_scheduler_enabled: Option<bool>,
    #[serde(rename = "BatchSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub batch_scheduler_enabled: Option<bool>,
    #[serde(rename = "ServiceSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub service_scheduler_enabled: Option<bool>,
}

impl PreemptionConfig {
    pub fn new() -> PreemptionConfig {
        PreemptionConfig {
            system_scheduler_enabled: None,
            batch_scheduler_enabled: None,
            service_scheduler_enabled: None,
        }
    }
}


