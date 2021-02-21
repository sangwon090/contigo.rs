use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::auth::*;
use crate::config;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthClient {
    email: String,
    password: String,
    device_uuid: String,
    device_name: String,
    os_version: String,
    permanent: bool,
    forced: bool,
    passcode: Option<String>,
}

#[derive(Debug)]
pub struct RegisterData {
    client: AuthClient,
    passcode: String,
}

#[derive(Debug)]
pub enum AuthResult {
    Success(Option<Credential>),
    DeviceNotRegistered,
    Error(i32, Option<String>),
}

impl AuthClient {
    pub fn new(account: Account, device: Device, permanent: bool, forced: bool) -> AuthClient {
        AuthClient {
            email: account.email,
            password: account.password,
            device_uuid: device.uuid,
            device_name: device.name,
            os_version: device.os_version,
            permanent,
            forced,
            passcode: None,
        }
    }

    pub async fn login(mut self) -> Result<AuthResult, Error> {
        self.passcode = None;

        let client = reqwest::Client::new();
        let res = client
            .post(format!("https://{}/win32/account/login.json", config::AUTH_HOST).as_str())
            .headers(AuthClient::get_header(&XVCKey::new(&self.email, &self.device_uuid), &self.os_version))
            .form(&self)
            .send()
            .await;

        match res {
            Ok(res) => {
                let result: Credential = serde_json::from_str(&res.text().await.unwrap()).unwrap();

                match result.status {
                    0 => Ok(AuthResult::Success(Some(result))),
                    _ => Ok(AuthResult::Error(result.status, result.message)),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn request_passcode(mut self) -> Result<AuthResult, Error> {
        self.passcode = None;

        let client = reqwest::Client::new();
        let res = client
            .post(format!("https://{}/win32/account/request_passcode.json", config::AUTH_HOST).as_str())
            .headers(AuthClient::get_header(&XVCKey::new(&self.email, &self.device_uuid), &self.os_version))
            .form(&self)
            .send()
            .await;

        match res {
            Ok(res) => {
                let result: Credential = serde_json::from_str(&res.text().await.unwrap()).unwrap();

                match result.status {
                    0 => Ok(AuthResult::Success(Some(result))),
                    _ => Ok(AuthResult::Error(result.status, result.message)),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn register_device(mut self, passcode: String) -> Result<AuthResult, Error> {
        self.passcode = Some(passcode);

        let client = reqwest::Client::new();
        let res = client
            .post(format!("https://{}/win32/account/register_device.json", config::AUTH_HOST).as_str())
            .headers(AuthClient::get_header(
                &XVCKey::new(&self.email, &self.device_uuid),
                &self.os_version,
            ))
            .form(&self)
            .send()
            .await;

        match res {
            Ok(res) => {
                let result: Credential = serde_json::from_str(&res.text().await.unwrap()).unwrap();

                match result.status {
                    0 => Ok(AuthResult::Success(Some(result))),
                    _ => Ok(AuthResult::Error(result.status, result.message)),
                }
            }
            Err(err) => Err(err),
        }
    }

    fn get_header(xvc_key: &str, os_version: &str) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::new();

        headers.append(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        headers.append(header::HOST, HeaderValue::from_str(config::AUTH_HOST).unwrap());
        headers.append("A", HeaderValue::from_str(&format!("{}/{}/{}", config::AUTH_AGENT, config::AUTH_VERSION, config::AUTH_LANG)).unwrap());
        headers.append("X-VC", HeaderValue::from_str(&xvc_key[0..16]).unwrap());
        headers.append(header::USER_AGENT, HeaderValue::from_str(&format!("KT/{} {}/{} {}", config::AUTH_VERSION, config::AUTH_PLATFORM, os_version, config::AUTH_LANG)).unwrap());
        headers.append(header::ACCEPT_LANGUAGE, HeaderValue::from_str(config::AUTH_LANG).unwrap());
        headers
    }
}
