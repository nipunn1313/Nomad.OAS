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
pub struct RegisterJobRequest {
    #[serde(rename = "Job")]
    pub job: Option<Box<crate::models::Job>>,
    #[serde(rename = "EnforceIndex", skip_serializing_if = "Option::is_none")]
    pub enforce_index: Option<bool>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "PolicyOverride", skip_serializing_if = "Option::is_none")]
    pub policy_override: Option<bool>,
}

impl RegisterJobRequest {
    pub fn new(job: Option<crate::models::Job>) -> RegisterJobRequest {
        RegisterJobRequest {
            job: None,
            enforce_index: None,
            job_modify_index: None,
            policy_override: None,
        }
    }
}


