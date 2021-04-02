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
pub struct JobListStub {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "ParentID", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<String>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Periodic", skip_serializing_if = "Option::is_none")]
    pub periodic: Option<bool>,
    #[serde(rename = "ParameterizedJob", skip_serializing_if = "Option::is_none")]
    pub parameterized_job: Option<bool>,
    #[serde(rename = "Stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "JobSummary", skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<Box<crate::models::JobSummary>>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "SubmitTime", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<i64>,
}

impl JobListStub {
    pub fn new() -> JobListStub {
        JobListStub {
            ID: None,
            parent_id: None,
            name: None,
            datacenters: None,
            _type: None,
            priority: None,
            periodic: None,
            parameterized_job: None,
            stop: None,
            status: None,
            status_description: None,
            job_summary: None,
            create_index: None,
            modify_index: None,
            job_modify_index: None,
            submit_time: None,
        }
    }
}


