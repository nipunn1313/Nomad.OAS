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
pub struct NodeReservedDiskResources {
    #[serde(rename = "DiskMB", skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<i32>,
}

impl NodeReservedDiskResources {
    pub fn new() -> NodeReservedDiskResources {
        NodeReservedDiskResources {
            disk_mb: None,
        }
    }
}


