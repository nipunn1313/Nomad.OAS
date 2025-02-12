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
pub struct CsiVolumeRegisterRequest {
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<crate::models::CsiVolume>>,
}

impl CsiVolumeRegisterRequest {
    pub fn new() -> CsiVolumeRegisterRequest {
        CsiVolumeRegisterRequest {
            volumes: None,
        }
    }
}


