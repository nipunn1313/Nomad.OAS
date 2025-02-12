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
pub struct ScalingPolicy {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl ScalingPolicy {
    pub fn new() -> ScalingPolicy {
        ScalingPolicy {
            ID: None,
            namespace: None,
            target: None,
            min: None,
            max: None,
            policy: None,
            enabled: None,
            create_index: None,
            modify_index: None,
        }
    }
}


