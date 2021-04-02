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
pub struct NodeDeviceLocality {
    #[serde(rename = "PciBusID", skip_serializing_if = "Option::is_none")]
    pub pci_bus_id: Option<String>,
}

impl NodeDeviceLocality {
    pub fn new() -> NodeDeviceLocality {
        NodeDeviceLocality {
            pci_bus_id: None,
        }
    }
}


