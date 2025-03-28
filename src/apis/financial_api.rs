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

/// struct for passing parameters to the method [`get_balance_info`]
#[derive(Clone, Debug)]
pub struct GetBalanceInfoParams {
    /// Sub-Account name
    pub sub_account_name: String,
    /// OAuth2.0 access token. Not required if using API key.
    pub authorization: Option<String>,
    /// API key. Not required if using access token.
    pub x_api_key: Option<String>,
    /// Name of requested coin. Default value is BTC.
    pub coin: Option<String>,
}

/// struct for passing parameters to the method [`get_earnings`]
#[derive(Clone, Debug)]
pub struct GetEarningsParams {
    /// Sub-Account name
    pub sub_account_name: String,
    /// API key. Not required if using access token.
    pub x_api_key: Option<String>,
    /// OAuth2.0 access token. Not required if using API key.
    pub authorization: Option<String>,
    /// Name of requested coin. Default value is BTC.
    pub coin: Option<String>,
    /// Start date inclusive, in unix epoch time (milliseconds). Default value is 0.
    pub start_date_unix_ms: Option<i64>,
    /// End date inclusive, in unix epoch time (milliseconds). Default value is current time.
    pub end_date_unix_ms: Option<i64>,
}

/// struct for passing parameters to the method [`get_financial_stats_for_sub_accounts`]
#[derive(Clone, Debug)]
pub struct GetFinancialStatsForSubAccountsParams {
    /// OAuth2.0 access token.
    pub authorization: String,
    /// User ID is optional. Defaults to logged in user
    pub user_id: String,
    /// Name of requested coin.
    pub coin: Option<String>,
    /// Start date inclusive, in unix epoch time (milliseconds). Default value is 0. Specify UTC start of the day epoch time.
    pub start_date_unix_ms: Option<i64>,
    /// End date inclusive, in unix epoch time (milliseconds). Default value is current time.
    pub end_date_unix_ms: Option<i64>,
    /// List of group ids.
    pub group_ids_list: Option<Vec<i32>>,
}

/// struct for passing parameters to the method [`get_transactions`]
#[derive(Clone, Debug)]
pub struct GetTransactionsParams {
    /// Sub-Account name
    pub sub_account_name: String,
    /// API key. Not required if using access token.
    pub x_api_key: Option<String>,
    /// OAuth2.0 access token. Not required if using API key.
    pub authorization: Option<String>,
    /// Name of requested coin. Default value is BTC.
    pub coin: Option<String>,
    /// Start date inclusive, in unix epoch time (milliseconds). Default value is 0.
    pub start_date_unix_ms: Option<i64>,
    /// End date inclusive, in unix epoch time (milliseconds). Default value is current time.
    pub end_date_unix_ms: Option<i64>,
}

/// struct for typed errors of method [`get_balance_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBalanceInfoError {
    Status404(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_earnings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEarningsError {
    Status404(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_financial_stats_for_sub_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFinancialStatsForSubAccountsError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionsError {
    Status404(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// Get balance, total earned, total fees and total withdrawn amount for a requested sub-account. Requires authorized user or admin authentication.
pub async fn get_balance_info(
    configuration: &configuration::Configuration,
    params: GetBalanceInfoParams,
) -> Result<models::BalanceResponse, Error<GetBalanceInfoError>> {
    let uri_str = format!(
        "{}/balance/{subAccountName}",
        configuration.base_path,
        subAccountName = crate::apis::urlencode(params.sub_account_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.authorization {
        req_builder = req_builder.header("Authorization", param_value.to_string());
    }
    if let Some(param_value) = params.x_api_key {
        req_builder = req_builder.header("X-API-KEY", param_value.to_string());
    }

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BalanceResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BalanceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBalanceInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get daily aggregated earnings for a requested sub-account. Requires authorized user or admin authentication.
pub async fn get_earnings(
    configuration: &configuration::Configuration,
    params: GetEarningsParams,
) -> Result<Vec<models::EarningResponse>, Error<GetEarningsError>> {
    let uri_str = format!(
        "{}/earnings/{subAccountName}",
        configuration.base_path,
        subAccountName = crate::apis::urlencode(params.sub_account_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.start_date_unix_ms {
        req_builder = req_builder.query(&[("startDateUnixMs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_date_unix_ms {
        req_builder = req_builder.query(&[("endDateUnixMs", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_api_key {
        req_builder = req_builder.header("X-API-KEY", param_value.to_string());
    }
    if let Some(param_value) = params.authorization {
        req_builder = req_builder.header("Authorization", param_value.to_string());
    }

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::EarningResponse&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::EarningResponse&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEarningsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get financial stats and their total for sub-accounts associated with a user. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.
pub async fn get_financial_stats_for_sub_accounts(
    configuration: &configuration::Configuration,
    params: GetFinancialStatsForSubAccountsParams,
) -> Result<models::SubAccountsFinancialStatsResponse, Error<GetFinancialStatsForSubAccountsError>>
{
    let uri_str = format!(
        "{}/financial_overview/{userId}",
        configuration.base_path,
        userId = crate::apis::urlencode(params.user_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.start_date_unix_ms {
        req_builder = req_builder.query(&[("startDateUnixMs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_date_unix_ms {
        req_builder = req_builder.query(&[("endDateUnixMs", &param_value.to_string())]);
    }
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SubAccountsFinancialStatsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SubAccountsFinancialStatsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFinancialStatsForSubAccountsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get transactions for a requested sub-account. Requires authorized user or admin authentication.
pub async fn get_transactions(
    configuration: &configuration::Configuration,
    params: GetTransactionsParams,
) -> Result<Vec<models::TransactionResponse>, Error<GetTransactionsError>> {
    let uri_str = format!(
        "{}/transactions/{subAccountName}",
        configuration.base_path,
        subAccountName = crate::apis::urlencode(params.sub_account_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.start_date_unix_ms {
        req_builder = req_builder.query(&[("startDateUnixMs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.end_date_unix_ms {
        req_builder = req_builder.query(&[("endDateUnixMs", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_api_key {
        req_builder = req_builder.header("X-API-KEY", param_value.to_string());
    }
    if let Some(param_value) = params.authorization {
        req_builder = req_builder.header("Authorization", param_value.to_string());
    }

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::TransactionResponse&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::TransactionResponse&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTransactionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
