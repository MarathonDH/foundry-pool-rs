/*
 * Foundry USA Pool API
 *
 * The Foundry USA Pool API allows users to view data and perform actions using custom written software. To get started, please follow the instructions in the Authentication Test endpoint.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogResponseV2 {
    #[serde(rename = "activityLogs")]
    pub activity_logs: Vec<models::ActivityLogEntryV2>,
    #[serde(rename = "totalLogCount")]
    pub total_log_count: i64,
    #[serde(rename = "groupName")]
    pub group_name: String,
}

impl ActivityLogResponseV2 {
    pub fn new(activity_logs: Vec<models::ActivityLogEntryV2>, total_log_count: i64, group_name: String) -> ActivityLogResponseV2 {
        ActivityLogResponseV2 {
            activity_logs,
            total_log_count,
            group_name,
        }
    }
}

