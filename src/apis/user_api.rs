/*
 * Foundry USA Pool API
 *
 * The Foundry USA Pool API allows users to view data and perform actions using custom written software. To get started, please follow the instructions in the Authentication Test endpoint.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`add_sub_account_user`]
#[derive(Clone, Debug)]
pub struct AddSubAccountUserParams {
    /// OAuth2.0 access token.
    pub authorization: models::String,
    /// User ID to be added to sub-account
    pub user_id: i32,
    /// Sub-Account name the user is to be added
    pub sub_account_name: String,
    /// Sub-Account role to be provided to this user. Valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".  Optional - Will default to user's defaultSubAccountRole if not included.
    pub add_user_to_sub_account_request: Option<models::AddUserToSubAccountRequest>
}

/// struct for passing parameters to the method [`deactivate_user`]
#[derive(Clone, Debug)]
pub struct DeactivateUserParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// Id of the user to be deactivated
    pub user_id: i32,
    /// List of group IDs to deactivate the user from
    pub group_ids_list: Vec<i32>
}

/// struct for passing parameters to the method [`delete_user_sub_account_role`]
#[derive(Clone, Debug)]
pub struct DeleteUserSubAccountRoleParams {
    /// OAuth2.0 access token.
    pub authorization: models::String,
    /// User ID associated with the role to be deleted
    pub user_id: i32,
    /// Sub-Account name the user's role is to be deleted from
    pub sub_account_name: String
}

/// struct for passing parameters to the method [`get_role_info_by_sub_account_name_and_user_id`]
#[derive(Clone, Debug)]
pub struct GetRoleInfoBySubAccountNameAndUserIdParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// The name of the sub-account
    pub sub_account_name: String,
    /// The ID of the user is optional. Defaults to logged in user.
    pub user_id: String
}

/// struct for passing parameters to the method [`get_user_by_logged_in_user`]
#[derive(Clone, Debug)]
pub struct GetUserByLoggedInUserParams {
    /// OAuth2.0 access token.
    pub authorization: models::String
}

/// struct for passing parameters to the method [`get_users_by_group_list`]
#[derive(Clone, Debug)]
pub struct GetUsersByGroupListParams {
    /// OAuth2.0 access token.
    pub authorization: models::String,
    /// Optional List of Group IDs.  If empty, returns all users in all groups the logged-in user has view permissions for.
    pub group_ids_list: Option<Vec<i32>>
}

/// struct for passing parameters to the method [`register`]
#[derive(Clone, Debug)]
pub struct RegisterParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// User info that needs to be added to the group
    pub new_user_request_v2: models::NewUserRequestV2
}

/// struct for passing parameters to the method [`update_all_user_sub_account_roles`]
#[derive(Clone, Debug)]
pub struct UpdateAllUserSubAccountRolesParams {
    /// OAuth2.0 access token.
    pub authorization: models::String,
    /// The ID of the user
    pub user_id: i32,
    /// The ID of the group
    pub group_id: i32,
    /// Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".
    pub new_sub_account_role_name: models::String
}

/// struct for passing parameters to the method [`update_user_sub_account_role`]
#[derive(Clone, Debug)]
pub struct UpdateUserSubAccountRoleParams {
    /// OAuth2.0 access token.
    pub authorization: models::String,
    /// The ID of the user
    pub user_id: i32,
    /// The name of the sub-account
    pub sub_account_name: String,
    /// Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".
    pub sub_account_role_name: String
}


/// struct for typed errors of method [`add_sub_account_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSubAccountUserError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deactivate_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeactivateUserError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_sub_account_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserSubAccountRoleError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_role_info_by_sub_account_name_and_user_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleInfoBySubAccountNameAndUserIdError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_by_logged_in_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserByLoggedInUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users_by_group_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersByGroupListError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`register`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterError {
    Status400(),
    Status403(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_all_user_sub_account_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAllUserSubAccountRolesError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_sub_account_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserSubAccountRoleError {
    Status400(),
    UnknownValue(serde_json::Value),
}


/// Add user to an existing sub-account. Requires admin write or owner permissions for the sub-account.
pub async fn add_sub_account_user(configuration: &configuration::Configuration, params: AddSubAccountUserParams) -> Result<(), Error<AddSubAccountUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let user_id = params.user_id;
    let sub_account_name = params.sub_account_name;
    let add_user_to_sub_account_request = params.add_user_to_sub_account_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{userId}/sub-account_role/{subAccountName}", local_var_configuration.base_path, userId=user_id, subAccountName=crate::apis::urlencode(sub_account_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&add_user_to_sub_account_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddSubAccountUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deactivate user associated with a list of groups or Create Approval Request to do so. Requires mining account owner authentication or admin write authentication.
pub async fn deactivate_user(configuration: &configuration::Configuration, params: DeactivateUserParams) -> Result<models::UserResponseV2, Error<DeactivateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let user_id = params.user_id;
    let group_ids_list = params.group_ids_list;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/users/{userId}", local_var_configuration.base_path, userId=user_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(&group_ids_list.into_iter().map(|p| ("groupIdsList".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("groupIdsList", &group_ids_list.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeactivateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete user’s role for a sub-account. Requires set role permissions for the sub-account.
pub async fn delete_user_sub_account_role(configuration: &configuration::Configuration, params: DeleteUserSubAccountRoleParams) -> Result<(), Error<DeleteUserSubAccountRoleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let user_id = params.user_id;
    let sub_account_name = params.sub_account_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{userId}/sub-account_role/{subAccountName}", local_var_configuration.base_path, userId=user_id, subAccountName=crate::apis::urlencode(sub_account_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteUserSubAccountRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the role of a user for a sub-account.
pub async fn get_role_info_by_sub_account_name_and_user_id(configuration: &configuration::Configuration, params: GetRoleInfoBySubAccountNameAndUserIdParams) -> Result<models::UserSubAccountRoleResponse, Error<GetRoleInfoBySubAccountNameAndUserIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let sub_account_name = params.sub_account_name;
    let user_id = params.user_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/get_role/{subAccountName}/{userId}", local_var_configuration.base_path, subAccountName=crate::apis::urlencode(sub_account_name), userId=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRoleInfoBySubAccountNameAndUserIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get info for user associated with the provided authentication token.
pub async fn get_user_by_logged_in_user(configuration: &configuration::Configuration, params: GetUserByLoggedInUserParams) -> Result<models::User, Error<GetUserByLoggedInUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/logged-in-user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserByLoggedInUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all users associated with a list of groups. Requires logged-in user to have access to, and view permissions for, all groups in list.
pub async fn get_users_by_group_list(configuration: &configuration::Configuration, params: GetUsersByGroupListParams) -> Result<Vec<models::GroupWithUsersResponseV2>, Error<GetUsersByGroupListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let group_ids_list = params.group_ids_list;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/users/users-by-groups", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = group_ids_list {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("groupIdsList".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("groupIdsList", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUsersByGroupListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a user and adds it to the groups that don't satisfy the approval request creation threshold to it. Else, create an approval request to do the same. Requires mining account owner or admin write authentication.
pub async fn register(configuration: &configuration::Configuration, params: RegisterParams) -> Result<models::UserResponseV2, Error<RegisterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let new_user_request_v2 = params.new_user_request_v2;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/users", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&new_user_request_v2);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RegisterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update user’s role for all sub-accounts, if your group doesn't satisfy the approval request creation threshold. Else, create an approval request to do the same. Requires mining account owner authentication.
pub async fn update_all_user_sub_account_roles(configuration: &configuration::Configuration, params: UpdateAllUserSubAccountRolesParams) -> Result<models::UserResponseV2, Error<UpdateAllUserSubAccountRolesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let user_id = params.user_id;
    let group_id = params.group_id;
    let new_sub_account_role_name = params.new_sub_account_role_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/users/{userId}/group/{groupId}/role/{newSubAccountRoleName}", local_var_configuration.base_path, userId=user_id, groupId=group_id, newSubAccountRoleName=new_sub_account_role_name.to_string());
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAllUserSubAccountRolesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update user’s role for a sub-account. Requires set role permissions for the sub-account.
pub async fn update_user_sub_account_role(configuration: &configuration::Configuration, params: UpdateUserSubAccountRoleParams) -> Result<i32, Error<UpdateUserSubAccountRoleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let user_id = params.user_id;
    let sub_account_name = params.sub_account_name;
    let sub_account_role_name = params.sub_account_role_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{userId}/sub-account_role/{subAccountName}/{subAccountRoleName}", local_var_configuration.base_path, userId=user_id, subAccountName=crate::apis::urlencode(sub_account_name), subAccountRoleName=crate::apis::urlencode(sub_account_role_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateUserSubAccountRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

