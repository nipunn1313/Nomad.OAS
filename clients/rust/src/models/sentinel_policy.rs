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
pub struct SentinelPolicy {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "EnforcementLevel", skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<String>,
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl SentinelPolicy {
    pub fn new() -> SentinelPolicy {
        SentinelPolicy {
            name: None,
            description: None,
            scope: None,
            enforcement_level: None,
            policy: None,
            create_index: None,
            modify_index: None,
        }
    }
}


