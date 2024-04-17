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
pub struct NewUserRequestV2 {
    /// Email Address of user
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// Full Name of new user - blank if adding an existing user to a group.
    #[serde(rename = "fullName")]
    pub full_name: String,
    /// Preferred Name for new user - blank if adding an existing user to a group.
    #[serde(rename = "preferredName")]
    pub preferred_name: String,
    /// List of UserGroupRequest objects
    #[serde(rename = "groups")]
    pub groups: Vec<models::UserGroupRequest>,
    /// True for API-Only users.
    #[serde(rename = "apiUser", skip_serializing_if = "Option::is_none")]
    pub api_user: Option<bool>,
    /// List of API User's IPs. Only valid if apiUser equals true.
    #[serde(rename = "ips", skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<String>>,
}

impl NewUserRequestV2 {
    pub fn new(email_address: String, full_name: String, preferred_name: String, groups: Vec<models::UserGroupRequest>) -> NewUserRequestV2 {
        NewUserRequestV2 {
            email_address,
            full_name,
            preferred_name,
            groups,
            api_user: None,
            ips: None,
        }
    }
}
