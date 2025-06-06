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
pub struct UserSubAccountResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "subAccountName")]
    pub sub_account_name: String,
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
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "approvers", skip_serializing_if = "Option::is_none")]
    pub approvers: Option<String>,
}

impl UserSubAccountResponse {
    pub fn new(
        user_id: i32,
        email_address: String,
        full_name: String,
        group_name: String,
        sub_account_name: String,
        role_name: String,
        default_sub_account_role_name: String,
        is_approval_action_required: bool,
    ) -> UserSubAccountResponse {
        UserSubAccountResponse {
            id: None,
            user_id,
            email_address,
            full_name,
            group_name,
            sub_account_name,
            role_name,
            default_sub_account_role_name,
            approval_request_id: None,
            is_approval_action_required,
            approval_threshold: None,
            status: None,
            approvers: None,
        }
    }
}
