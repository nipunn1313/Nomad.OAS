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
pub struct NodeScoreMeta {
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "Scores", skip_serializing_if = "Option::is_none")]
    pub scores: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "NormScore", skip_serializing_if = "Option::is_none")]
    pub norm_score: Option<f64>,
}

impl NodeScoreMeta {
    pub fn new() -> NodeScoreMeta {
        NodeScoreMeta {
            node_id: None,
            scores: None,
            norm_score: None,
        }
    }
}


