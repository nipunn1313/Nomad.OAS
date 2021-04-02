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
pub struct Resources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "MemoryMb", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[serde(rename = "DiskMb", skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<i32>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::RequestedDevice>>,
    #[serde(rename = "Iops", skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            cpu: None,
            memory_mb: None,
            disk_mb: None,
            networks: None,
            devices: None,
            iops: None,
        }
    }
}


