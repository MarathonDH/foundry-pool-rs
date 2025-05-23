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
pub struct UserRole {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "createSubAccounts")]
    pub create_sub_accounts: bool,
    #[serde(rename = "manageUsers")]
    pub manage_users: bool,
    #[serde(rename = "approve")]
    pub approve: bool,
}

impl UserRole {
    pub fn new(
        id: i32,
        role_name: String,
        create_sub_accounts: bool,
        manage_users: bool,
        approve: bool,
    ) -> UserRole {
        UserRole {
            id,
            role_name,
            create_sub_accounts,
            manage_users,
            approve,
        }
    }
}
