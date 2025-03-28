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
pub struct UserGroupResponse {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "defaultSubAccountRoleName")]
    pub default_sub_account_role_name: String,
    #[serde(rename = "approvalRequestId", skip_serializing_if = "Option::is_none")]
    pub approval_request_id: Option<i32>,
    #[serde(rename = "isApprovalActionRequired")]
    pub is_approval_action_required: bool,
    #[serde(rename = "approvalThreshold", skip_serializing_if = "Option::is_none")]
    pub approval_threshold: Option<i32>,
    #[serde(rename = "approvalStatus", skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    #[serde(rename = "approvers", skip_serializing_if = "Option::is_none")]
    pub approvers: Option<String>,
    #[serde(
        rename = "approvalActionRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_action_required: Option<bool>,
}

impl UserGroupResponse {
    pub fn new(
        group_name: String,
        role_name: String,
        default_sub_account_role_name: String,
        is_approval_action_required: bool,
    ) -> UserGroupResponse {
        UserGroupResponse {
            group_name,
            role_name,
            default_sub_account_role_name,
            approval_request_id: None,
            is_approval_action_required,
            approval_threshold: None,
            approval_status: None,
            approvers: None,
            approval_action_required: None,
        }
    }
}
