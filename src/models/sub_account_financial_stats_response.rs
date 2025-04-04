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
pub struct SubAccountFinancialStatsResponse {
    #[serde(rename = "statsDate")]
    pub stats_date: String,
    #[serde(rename = "subAccountName")]
    pub sub_account_name: String,
    #[serde(rename = "hashratePrevDay")]
    pub hashrate_prev_day: i64,
    #[serde(rename = "ppsBaseAmount")]
    pub pps_base_amount: f64,
    #[serde(rename = "txFeeRewardAmount")]
    pub tx_fee_reward_amount: f64,
    #[serde(rename = "feeAmount")]
    pub fee_amount: f64,
    #[serde(rename = "totalEarnedInDuration")]
    pub total_earned_in_duration: f64,
    #[serde(rename = "totalFeesInDuration")]
    pub total_fees_in_duration: f64,
    #[serde(rename = "totalWithdrawnInDuration")]
    pub total_withdrawn_in_duration: f64,
    #[serde(rename = "earningsInfo")]
    pub earnings_info: Box<models::BalanceResponse>,
    #[serde(rename = "autoWithdraw")]
    pub auto_withdraw: bool,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "groupName")]
    pub group_name: String,
}

impl SubAccountFinancialStatsResponse {
    pub fn new(
        stats_date: String,
        sub_account_name: String,
        hashrate_prev_day: i64,
        pps_base_amount: f64,
        tx_fee_reward_amount: f64,
        fee_amount: f64,
        total_earned_in_duration: f64,
        total_fees_in_duration: f64,
        total_withdrawn_in_duration: f64,
        earnings_info: models::BalanceResponse,
        auto_withdraw: bool,
        group_id: i32,
        group_name: String,
    ) -> SubAccountFinancialStatsResponse {
        SubAccountFinancialStatsResponse {
            stats_date,
            sub_account_name,
            hashrate_prev_day,
            pps_base_amount,
            tx_fee_reward_amount,
            fee_amount,
            total_earned_in_duration,
            total_fees_in_duration,
            total_withdrawn_in_duration,
            earnings_info: Box::new(earnings_info),
            auto_withdraw,
            group_id,
            group_name,
        }
    }
}
