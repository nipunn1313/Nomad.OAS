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
pub struct JobRevertRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "JobVersion", skip_serializing_if = "Option::is_none")]
    pub job_version: Option<i32>,
    #[serde(rename = "EnforcePriorVersion", skip_serializing_if = "Option::is_none")]
    pub enforce_prior_version: Option<i32>,
    #[serde(rename = "ConsulToken", skip_serializing_if = "Option::is_none")]
    pub consul_token: Option<String>,
    #[serde(rename = "VaultToken", skip_serializing_if = "Option::is_none")]
    pub vault_token: Option<String>,
}

impl JobRevertRequest {
    pub fn new(job_id: String) -> JobRevertRequest {
        JobRevertRequest {
            job_id,
            job_version: None,
            enforce_prior_version: None,
            consul_token: None,
            vault_token: None,
        }
    }
}


