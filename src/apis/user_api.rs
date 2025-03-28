/*
 * Foundry USA Pool API
 *
 * The Foundry USA Pool API allows users to view data and perform actions using custom written software. To get started, please follow the instructions in the Authentication Test endpoint.
 *
 * The version of the OpenAPI document: 6.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`add_sub_account_user`]
#[derive(Clone, Debug)]
pub struct AddSubAccountUserParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// User ID to be added to sub-account
    pub user_id: i32,
    /// Sub-Account name the user is to be added
    pub sub_account_name: String,
    /// Sub-Account role to be provided to this user. Valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".  Optional - Will default to user's defaultSubAccountRole if not included.
    pub add_user_to_sub_account_request: Option<models::AddUserToSubAccountRequest>,
}

/// struct for passing parameters to the method [`deactivate_user`]
#[derive(Clone, Debug)]
pub struct DeactivateUserParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// Id of the user to be deactivated
    pub user_id: i32,
    /// List of group IDs to deactivate the user from
    pub group_ids_list: Vec<i32>,
}

/// struct for passing parameters to the method [`delete_user_sub_account_role`]
#[derive(Clone, Debug)]
pub struct DeleteUserSubAccountRoleParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// User ID associated with the role to be deleted
    pub user_id: i32,
    /// Sub-Account name the user's role is to be deleted from
    pub sub_account_name: String,
}

/// struct for passing parameters to the method [`get_role_info_by_sub_account_name_and_user_id`]
#[derive(Clone, Debug)]
pub struct GetRoleInfoBySubAccountNameAndUserIdParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// The name of the sub-account
    pub sub_account_name: String,
    /// The ID of the user is optional. Defaults to logged in user.
    pub user_id: String,
}

/// struct for passing parameters to the method [`get_user_by_logged_in_user`]
#[derive(Clone, Debug)]
pub struct GetUserByLoggedInUserParams {
    /// OAuth2.0 access token.
    pub authorization: String,
}

/// struct for passing parameters to the method [`get_users_by_group`]
#[derive(Clone, Debug)]
pub struct GetUsersByGroupParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// Group ID.
    pub group_id: i32,
    /// Optional page number.  Defaults to 0.
    pub page_number: Option<i32>,
    /// Optional page size.  If empty, returns all results.
    pub page_size: Option<i32>,
    /// Optional sort value.  Options are 'fullNameAsc', 'fullNameDesc', 'emailAddressAsc', 'emailAddressDesc', 'defaultRoleAsc', or 'defaultRoleDesc'.  If empty, defaults to fullNameAsc.
    pub sort: Option<String>,
    /// Optional user name filter value.  If empty, returns all results.
    pub user_full_name: Option<String>,
}

/// struct for passing parameters to the method [`get_users_by_group_list`]
#[derive(Clone, Debug)]
pub struct GetUsersByGroupListParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// Optional List of Group IDs.  If empty, returns all users in all groups the logged-in user has view permissions for.
    pub group_ids_list: Option<Vec<i32>>,
}

/// struct for passing parameters to the method [`register`]
#[derive(Clone, Debug)]
pub struct RegisterParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// User info that needs to be added to the group
    pub new_user_request_v2: models::NewUserRequestV2,
}

/// struct for passing parameters to the method [`update_all_user_sub_account_roles`]
#[derive(Clone, Debug)]
pub struct UpdateAllUserSubAccountRolesParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// The ID of the user
    pub user_id: i32,
    /// The ID of the group
    pub group_id: i32,
    /// Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".
    pub new_sub_account_role_name: String,
}

/// struct for passing parameters to the method [`update_user_sub_account_role`]
#[derive(Clone, Debug)]
pub struct UpdateUserSubAccountRoleParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// The ID of the user
    pub user_id: i32,
    /// The name of the sub-account
    pub sub_account_name: String,
    /// Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".
    pub sub_account_role_name: String,
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
    Status403(),
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

/// struct for typed errors of method [`get_users_by_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersByGroupError {
    Status403(),
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
    Status404(),
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
pub async fn add_sub_account_user(
    configuration: &configuration::Configuration,
    params: AddSubAccountUserParams,
) -> Result<(), Error<AddSubAccountUserError>> {
    let uri_str = format!(
        "{}/users/{userId}/sub-account_role/{subAccountName}",
        configuration.base_path,
        userId = params.user_id,
        subAccountName = crate::apis::urlencode(params.sub_account_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());
    req_builder = req_builder.json(&params.add_user_to_sub_account_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AddSubAccountUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Deactivate user associated with a list of groups or Create Approval Request to do so. Requires mining account owner authentication or admin write authentication.
pub async fn deactivate_user(
    configuration: &configuration::Configuration,
    params: DeactivateUserParams,
) -> Result<models::UserResponseV2, Error<DeactivateUserError>> {
    let uri_str = format!(
        "{}/v2/users/{userId}",
        configuration.base_path,
        userId = params.user_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = match "multi" {
        "multi" => req_builder.query(
            &params
                .group_ids_list
                .into_iter()
                .map(|p| ("groupIdsList".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => req_builder.query(&[(
            "groupIdsList",
            &params
                .group_ids_list
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserResponseV2`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserResponseV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeactivateUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete user’s role for a sub-account. Requires set role permissions for the sub-account.
pub async fn delete_user_sub_account_role(
    configuration: &configuration::Configuration,
    params: DeleteUserSubAccountRoleParams,
) -> Result<(), Error<DeleteUserSubAccountRoleError>> {
    let uri_str = format!(
        "{}/users/{userId}/sub-account_role/{subAccountName}",
        configuration.base_path,
        userId = params.user_id,
        subAccountName = crate::apis::urlencode(params.sub_account_name)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteUserSubAccountRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get the role of a user for a sub-account.
pub async fn get_role_info_by_sub_account_name_and_user_id(
    configuration: &configuration::Configuration,
    params: GetRoleInfoBySubAccountNameAndUserIdParams,
) -> Result<models::UserSubAccountRoleResponse, Error<GetRoleInfoBySubAccountNameAndUserIdError>> {
    let uri_str = format!(
        "{}/get_role/{subAccountName}/{userId}",
        configuration.base_path,
        subAccountName = crate::apis::urlencode(params.sub_account_name),
        userId = crate::apis::urlencode(params.user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserSubAccountRoleResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserSubAccountRoleResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRoleInfoBySubAccountNameAndUserIdError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get info for user associated with the provided authentication token.
pub async fn get_user_by_logged_in_user(
    configuration: &configuration::Configuration,
    params: GetUserByLoggedInUserParams,
) -> Result<models::User, Error<GetUserByLoggedInUserError>> {
    let uri_str = format!("{}/users/logged-in-user", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::User`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::User`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUserByLoggedInUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get all users associated with a group. Requires logged-in user to have access to and view permissions for group.
pub async fn get_users_by_group(
    configuration: &configuration::Configuration,
    params: GetUsersByGroupParams,
) -> Result<Vec<models::GroupWithUsersResponseV2>, Error<GetUsersByGroupError>> {
    let uri_str = format!("{}/v2/users/users-by-group", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("groupId", &params.group_id.to_string())]);
    if let Some(ref param_value) = params.page_number {
        req_builder = req_builder.query(&[("pageNumber", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.user_full_name {
        req_builder = req_builder.query(&[("userFullName", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GroupWithUsersResponseV2&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GroupWithUsersResponseV2&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUsersByGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get all users associated with a list of groups. Requires logged-in user to have access to, and view permissions for, all groups in list.
pub async fn get_users_by_group_list(
    configuration: &configuration::Configuration,
    params: GetUsersByGroupListParams,
) -> Result<Vec<models::GroupWithUsersResponseV2>, Error<GetUsersByGroupListError>> {
    let uri_str = format!("{}/v2/users/users-by-groups", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.group_ids_list {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("groupIdsList".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "groupIdsList",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GroupWithUsersResponseV2&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GroupWithUsersResponseV2&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUsersByGroupListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Creates a user and adds it to the groups that don't satisfy the approval request creation threshold to it. Else, create an approval request to do the same. Requires mining account owner or admin write authentication.
pub async fn register(
    configuration: &configuration::Configuration,
    params: RegisterParams,
) -> Result<models::UserResponseV2, Error<RegisterError>> {
    let uri_str = format!("{}/v2/users", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());
    req_builder = req_builder.json(&params.new_user_request_v2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserResponseV2`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserResponseV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RegisterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update user’s role for all sub-accounts, if your group doesn't satisfy the approval request creation threshold. Else, create an approval request to do the same. Requires mining account owner authentication.
pub async fn update_all_user_sub_account_roles(
    configuration: &configuration::Configuration,
    params: UpdateAllUserSubAccountRolesParams,
) -> Result<models::UserResponseV2, Error<UpdateAllUserSubAccountRolesError>> {
    let uri_str = format!(
        "{}/v2/users/{userId}/group/{groupId}/role/{newSubAccountRoleName}",
        configuration.base_path,
        userId = params.user_id,
        groupId = params.group_id,
        newSubAccountRoleName = crate::apis::urlencode(params.new_sub_account_role_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserResponseV2`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserResponseV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAllUserSubAccountRolesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update user’s role for a sub-account. Requires set role permissions for the sub-account.
pub async fn update_user_sub_account_role(
    configuration: &configuration::Configuration,
    params: UpdateUserSubAccountRoleParams,
) -> Result<i32, Error<UpdateUserSubAccountRoleError>> {
    let uri_str = format!(
        "{}/users/{userId}/sub-account_role/{subAccountName}/{subAccountRoleName}",
        configuration.base_path,
        userId = params.user_id,
        subAccountName = crate::apis::urlencode(params.sub_account_name),
        subAccountRoleName = crate::apis::urlencode(params.sub_account_role_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", params.authorization.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `i32`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `i32`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateUserSubAccountRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
