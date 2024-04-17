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
pub struct AccessToken {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    #[serde(rename = "token_type")]
    pub token_type: String,
}

impl AccessToken {
    pub fn new(access_token: String, expires_in: i32, token_type: String) -> AccessToken {
        AccessToken {
            access_token,
            expires_in,
            token_type,
        }
    }
}
