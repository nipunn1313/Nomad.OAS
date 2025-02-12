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
pub struct NodeReservedNetworkResources {
    #[serde(rename = "ReservedHostPorts", skip_serializing_if = "Option::is_none")]
    pub reserved_host_ports: Option<String>,
}

impl NodeReservedNetworkResources {
    pub fn new() -> NodeReservedNetworkResources {
        NodeReservedNetworkResources {
            reserved_host_ports: None,
        }
    }
}


