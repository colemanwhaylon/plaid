use std::env;
use std::time::Duration;

use reqwest::Client as ReqwestClient;
use reqwest::Error as ReqwestError;
use serde_json::json;

use crate::types::*;

/// **[Plaid](https://plaid.com/docs) API client**.
///
/// See official documentation at: [https://plaid.com/docs](https://plaid.com/docs).
#[allow(dead_code)]
pub struct Client {
    client_id: String,
    secret: Secret,
    url: String,
    client: ReqwestClient,
}


impl Client {
    /// Creates a new `Client`.
    #[allow(dead_code)]
    pub fn new<C, S>(client_id: C, secret: S, environment: Environment) -> Client
    where
        C: Into<String>,
        S: Into<Secret>,
    {
        Client {
            client_id: client_id.into(),
            secret: secret.into(),
            url: format!("https://{}.plaid.com", environment),
            client: ReqwestClient::builder()
                .connect_timeout(Duration::from_secs(30))
                .build()
                .expect("could not create Reqwest client"),
        }
    }



}

