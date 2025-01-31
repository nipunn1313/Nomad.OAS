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
pub struct JobChildrenSummary {
    #[serde(rename = "Pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<i64>,
}

impl JobChildrenSummary {
    pub fn new() -> JobChildrenSummary {
        JobChildrenSummary {
            pending: None,
            running: None,
            dead: None,
        }
    }
}


