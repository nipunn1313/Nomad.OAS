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
pub struct CsiControllerInfo {
    #[serde(rename = "SupportsReadOnlyAttach", skip_serializing_if = "Option::is_none")]
    pub supports_read_only_attach: Option<bool>,
    #[serde(rename = "SupportsAttachDetach", skip_serializing_if = "Option::is_none")]
    pub supports_attach_detach: Option<bool>,
    #[serde(rename = "SupportsListVolumes", skip_serializing_if = "Option::is_none")]
    pub supports_list_volumes: Option<bool>,
    #[serde(rename = "SupportsListVolumesAttachedNodes", skip_serializing_if = "Option::is_none")]
    pub supports_list_volumes_attached_nodes: Option<bool>,
}

impl CsiControllerInfo {
    pub fn new() -> CsiControllerInfo {
        CsiControllerInfo {
            supports_read_only_attach: None,
            supports_attach_detach: None,
            supports_list_volumes: None,
            supports_list_volumes_attached_nodes: None,
        }
    }
}


