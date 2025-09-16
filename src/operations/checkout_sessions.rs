mod builder;
mod config;

pub use config::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub use builder::*;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CheckoutSessions;

impl CheckoutSessions {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CheckoutSessionsConfig,
    ) -> Result<CheckoutSessionsResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, "/checkouts", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionsResponse {
    pub session_id: String,
    pub checkout_url: String,
}
