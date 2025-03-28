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
pub struct WalletAllocationRequest {
    /// Sub Account Wallet id of the wallet
    #[serde(rename = "subAccountWalletId")]
    pub sub_account_wallet_id: i32,
    /// Allocation percentage for this wallet. Up to 3 decimals of precision.
    #[serde(rename = "allocationPercent")]
    pub allocation_percent: f64,
    /// Auto Withdraw toggle
    #[serde(rename = "autoWithdraw")]
    pub auto_withdraw: bool,
}

impl WalletAllocationRequest {
    pub fn new(
        sub_account_wallet_id: i32,
        allocation_percent: f64,
        auto_withdraw: bool,
    ) -> WalletAllocationRequest {
        WalletAllocationRequest {
            sub_account_wallet_id,
            allocation_percent,
            auto_withdraw,
        }
    }
}
