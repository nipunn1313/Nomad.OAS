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
pub struct TaskCsiPluginConfig {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "MountDir", skip_serializing_if = "Option::is_none")]
    pub mount_dir: Option<String>,
}

impl TaskCsiPluginConfig {
    pub fn new() -> TaskCsiPluginConfig {
        TaskCsiPluginConfig {
            ID: None,
            _type: None,
            mount_dir: None,
        }
    }
}


