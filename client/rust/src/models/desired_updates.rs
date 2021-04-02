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
pub struct DesiredUpdates {
    #[serde(rename = "Ignore", skip_serializing_if = "Option::is_none")]
    pub ignore: Option<i32>,
    #[serde(rename = "Place", skip_serializing_if = "Option::is_none")]
    pub place: Option<i32>,
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<i32>,
    #[serde(rename = "Stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
    #[serde(rename = "InPlaceUpdate", skip_serializing_if = "Option::is_none")]
    pub in_place_update: Option<i32>,
    #[serde(rename = "DestructiveUpdate", skip_serializing_if = "Option::is_none")]
    pub destructive_update: Option<i32>,
    #[serde(rename = "Canary", skip_serializing_if = "Option::is_none")]
    pub canary: Option<i32>,
    #[serde(rename = "Preemptions", skip_serializing_if = "Option::is_none")]
    pub preemptions: Option<i32>,
}

impl DesiredUpdates {
    pub fn new() -> DesiredUpdates {
        DesiredUpdates {
            ignore: None,
            place: None,
            migrate: None,
            stop: None,
            in_place_update: None,
            destructive_update: None,
            canary: None,
            preemptions: None,
        }
    }
}


