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
pub struct CsiPlugin {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "Controllers", skip_serializing_if = "Option::is_none")]
    pub controllers: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "Nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "Allocations", skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<crate::models::AllocationListStub>>,
    #[serde(rename = "ControllersHealthy", skip_serializing_if = "Option::is_none")]
    pub controllers_healthy: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl CsiPlugin {
    pub fn new() -> CsiPlugin {
        CsiPlugin {
            ID: None,
            provider: None,
            version: None,
            controller_required: None,
            controllers: None,
            nodes: None,
            allocations: None,
            controllers_healthy: None,
            nodes_healthy: None,
            create_index: None,
            modify_index: None,
        }
    }
}


