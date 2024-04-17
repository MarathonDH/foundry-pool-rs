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

/// struct for passing parameters to the method [`get_activity_log_for_groups`]
#[derive(Clone, Debug)]
pub struct GetActivityLogForGroupsParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// List of Group IDs is optional. If no list is passed, all groups the logged-in user has view permissions for is returned.
    pub group_ids_list: Option<Vec<i32>>,
    /// Start date inclusive, in unix epoch time (milliseconds). Default value is 30 days ago.
    pub start_date_unix_ms: Option<i64>,
    /// End date inclusive, in unix epoch time (milliseconds). Default value is current time.
    pub end_date_unix_ms: Option<i64>,
    /// Valid values are 0 and positive integers.
    pub page_number: Option<i32>,
    /// Valid values are -1 (representing max size) and positive integers.
    pub page_size: Option<i32>,
    /// Filter logs by coin.
    pub coin: Option<String>,
    /// Filter logs by activity type. List of available activity types can be GET from /activity_log/activity_types endpoint
    pub activity_type: Option<String>,
    /// Filter logs by a user's email address
    pub user_email: Option<String>,
    /// Filter logs by a subAccountName for a Group in which that SubAccount exists
    pub sub_account_name: Option<String>,
    /// Sort the logs by \"newest\" or \"oldest\" case-sensitive.
    pub sort: Option<String>,
    /// Filter logs by hiding all auth0 activity types.
    pub hide_auth_logs: Option<bool>
}

/// struct for passing parameters to the method [`get_activity_log_types`]
#[derive(Clone, Debug)]
pub struct GetActivityLogTypesParams {
    /// OAuth2.0 access token.
    pub authorization: String
}


/// struct for typed errors of method [`get_activity_log_for_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActivityLogForGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_activity_log_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActivityLogTypesError {
    UnknownValue(serde_json::Value),
}


/// Get activity log for a list of groups. Mining account owner or accountant authentication required to see own groups activity log. Admin authentication with read permission required to see other groups activity log.
pub async fn get_activity_log_for_groups(configuration: &configuration::Configuration, params: GetActivityLogForGroupsParams) -> Result<models::ActivityLogResponseV2WithTotal, Error<GetActivityLogForGroupsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;
    let group_ids_list = params.group_ids_list;
    let start_date_unix_ms = params.start_date_unix_ms;
    let end_date_unix_ms = params.end_date_unix_ms;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let coin = params.coin;
    let activity_type = params.activity_type;
    let user_email = params.user_email;
    let sub_account_name = params.sub_account_name;
    let sort = params.sort;
    let hide_auth_logs = params.hide_auth_logs;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/activity_log", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = group_ids_list {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("groupIdsList".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("groupIdsList", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = start_date_unix_ms {
        local_var_req_builder = local_var_req_builder.query(&[("startDateUnixMs", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date_unix_ms {
        local_var_req_builder = local_var_req_builder.query(&[("endDateUnixMs", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder = local_var_req_builder.query(&[("pageNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = coin {
        local_var_req_builder = local_var_req_builder.query(&[("coin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = activity_type {
        local_var_req_builder = local_var_req_builder.query(&[("activityType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_email {
        local_var_req_builder = local_var_req_builder.query(&[("userEmail", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sub_account_name {
        local_var_req_builder = local_var_req_builder.query(&[("subAccountName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = hide_auth_logs {
        local_var_req_builder = local_var_req_builder.query(&[("hideAuthLogs", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetActivityLogForGroupsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get list of activities types that are currently being logged by the pool.
pub async fn get_activity_log_types(configuration: &configuration::Configuration, params: GetActivityLogTypesParams) -> Result<Vec<models::ActivityTypesEnumResponse>, Error<GetActivityLogTypesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let authorization = params.authorization;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/activity_log/activity_types", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetActivityLogTypesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

