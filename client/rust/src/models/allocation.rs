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
pub struct Allocation {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "EvalID", skip_serializing_if = "Option::is_none")]
    pub eval_id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeName", skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::models::Job>>,
    #[serde(rename = "TaskGroup", skip_serializing_if = "Option::is_none")]
    pub task_group: Option<String>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<crate::models::Resources>>,
    #[serde(rename = "TaskResources", skip_serializing_if = "Option::is_none")]
    pub task_resources: Option<::std::collections::HashMap<String, crate::models::Resources>>,
    #[serde(rename = "AllocatedResources", skip_serializing_if = "Option::is_none")]
    pub allocated_resources: Option<Box<crate::models::AllocatedResources>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<crate::models::AllocationMetric>>,
    #[serde(rename = "DesiredStatus", skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    #[serde(rename = "DesiredDescription", skip_serializing_if = "Option::is_none")]
    pub desired_description: Option<String>,
    #[serde(rename = "DesiredTransition", skip_serializing_if = "Option::is_none")]
    pub desired_transition: Option<Box<crate::models::DesiredTransition>>,
    #[serde(rename = "ClientStatus", skip_serializing_if = "Option::is_none")]
    pub client_status: Option<String>,
    #[serde(rename = "ClientDescription", skip_serializing_if = "Option::is_none")]
    pub client_description: Option<String>,
    #[serde(rename = "TaskStates", skip_serializing_if = "Option::is_none")]
    pub task_states: Option<::std::collections::HashMap<String, crate::models::TaskState>>,
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentStatus", skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<Box<crate::models::AllocDeploymentStatus>>,
    #[serde(rename = "FollowupEvalID", skip_serializing_if = "Option::is_none")]
    pub followup_eval_id: Option<String>,
    #[serde(rename = "PreviousAllocation", skip_serializing_if = "Option::is_none")]
    pub previous_allocation: Option<String>,
    #[serde(rename = "NextAllocation", skip_serializing_if = "Option::is_none")]
    pub next_allocation: Option<String>,
    #[serde(rename = "RescheduleTracker", skip_serializing_if = "Option::is_none")]
    pub reschedule_tracker: Option<Box<crate::models::RescheduleTracker>>,
    #[serde(rename = "PreemptedAllocations", skip_serializing_if = "Option::is_none")]
    pub preempted_allocations: Option<Vec<String>>,
    #[serde(rename = "PreemptedByAllocation", skip_serializing_if = "Option::is_none")]
    pub preempted_by_allocation: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "AllocModifyIndex", skip_serializing_if = "Option::is_none")]
    pub alloc_modify_index: Option<i32>,
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "ModifyTime", skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

impl Allocation {
    pub fn new() -> Allocation {
        Allocation {
            ID: None,
            namespace: None,
            eval_id: None,
            name: None,
            node_id: None,
            node_name: None,
            job_id: None,
            job: None,
            task_group: None,
            resources: None,
            task_resources: None,
            allocated_resources: None,
            services: None,
            metrics: None,
            desired_status: None,
            desired_description: None,
            desired_transition: None,
            client_status: None,
            client_description: None,
            task_states: None,
            deployment_id: None,
            deployment_status: None,
            followup_eval_id: None,
            previous_allocation: None,
            next_allocation: None,
            reschedule_tracker: None,
            preempted_allocations: None,
            preempted_by_allocation: None,
            create_index: None,
            modify_index: None,
            alloc_modify_index: None,
            create_time: None,
            modify_time: None,
        }
    }
}


