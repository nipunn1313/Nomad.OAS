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
pub struct VolumeRequest {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "MountOptions", skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Box<crate::models::CsiMountOptions>>,
}

impl VolumeRequest {
    pub fn new() -> VolumeRequest {
        VolumeRequest {
            name: None,
            _type: None,
            source: None,
            read_only: None,
            mount_options: None,
        }
    }
}


