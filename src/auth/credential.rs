use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Credential {
    pub status: i32,
    pub message: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "countryIso")]
    pub country_iso: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: Option<i32>,
    pub server_time: Option<i32>,
    #[serde(rename = "resetUserData")]
    pub reset_user_data: Option<bool>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    #[serde(rename = "autoLoginAccountId")]
    pub auto_login_id: Option<String>,
    #[serde(rename = "displayAccountId")]
    pub display_id: Option<String>,
    #[serde(rename = "mainDeviceAgentName")]
    pub main_device_name: Option<String>,
    #[serde(rename = "mainDeviceAppVersion")]
    pub main_device_app_version: Option<String>,
}
