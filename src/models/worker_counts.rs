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
pub struct WorkerCounts {
    #[serde(rename = "workerCount")]
    pub worker_count: i32,
    #[serde(rename = "offline15MinWorkerCount")]
    pub offline15_min_worker_count: i32,
    #[serde(rename = "offline24HrWorkerCount")]
    pub offline24_hr_worker_count: i32,
}

impl WorkerCounts {
    pub fn new(worker_count: i32, offline15_min_worker_count: i32, offline24_hr_worker_count: i32) -> WorkerCounts {
        WorkerCounts {
            worker_count,
            offline15_min_worker_count,
            offline24_hr_worker_count,
        }
    }
}

