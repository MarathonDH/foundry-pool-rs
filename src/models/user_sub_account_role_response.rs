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
pub struct UserSubAccountRoleResponse {
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "viewHashrate")]
    pub view_hashrate: bool,
    #[serde(rename = "viewPayments")]
    pub view_payments: bool,
    #[serde(rename = "withdraw")]
    pub withdraw: bool,
    #[serde(rename = "setRoles")]
    pub set_roles: bool,
    #[serde(rename = "editWorkers")]
    pub edit_workers: bool,
    #[serde(rename = "manageApiKeys")]
    pub manage_api_keys: bool,
    #[serde(rename = "approve")]
    pub approve: bool,
}

impl UserSubAccountRoleResponse {
    pub fn new(role_name: String, view_hashrate: bool, view_payments: bool, withdraw: bool, set_roles: bool, edit_workers: bool, manage_api_keys: bool, approve: bool) -> UserSubAccountRoleResponse {
        UserSubAccountRoleResponse {
            role_name,
            view_hashrate,
            view_payments,
            withdraw,
            set_roles,
            edit_workers,
            manage_api_keys,
            approve,
        }
    }
}
