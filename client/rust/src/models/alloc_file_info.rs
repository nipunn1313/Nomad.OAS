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
pub struct AllocFileInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "IsDir", skip_serializing_if = "Option::is_none")]
    pub is_dir: Option<bool>,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "FileMode", skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "ModTime", skip_serializing_if = "Option::is_none")]
    pub mod_time: Option<String>,
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl AllocFileInfo {
    pub fn new() -> AllocFileInfo {
        AllocFileInfo {
            name: None,
            is_dir: None,
            size: None,
            file_mode: None,
            mod_time: None,
            content_type: None,
        }
    }
}


