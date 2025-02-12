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
pub struct ObjectDiff {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::FieldDiff>>,
    #[serde(rename = "Objects", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<crate::models::ObjectDiff>>,
}

impl ObjectDiff {
    pub fn new() -> ObjectDiff {
        ObjectDiff {
            _type: None,
            name: None,
            fields: None,
            objects: None,
        }
    }
}


