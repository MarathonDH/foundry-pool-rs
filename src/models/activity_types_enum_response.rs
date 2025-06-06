/*
 * Foundry USA Pool API
 *
 * The Foundry USA Pool API allows users to view data and perform actions using custom written software. To get started, please follow the instructions in the Authentication Test endpoint.
 *
 * The version of the OpenAPI document: 6.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityTypesEnumResponse {
    #[serde(rename = "activityName")]
    pub activity_name: String,
    #[serde(rename = "description")]
    pub description: String,
}

impl ActivityTypesEnumResponse {
    pub fn new(activity_name: String, description: String) -> ActivityTypesEnumResponse {
        ActivityTypesEnumResponse {
            activity_name,
            description,
        }
    }
}
