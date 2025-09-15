mod builder;
mod config;

pub use config::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub use builder::*;

use crate::client::Handle;

pub struct CheckoutSessions;

impl CheckoutSessions {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CheckoutSessionsConfig,
    ) -> CheckoutSessionsResponse {
        let body = serde_json::to_string(&config).unwrap();
        let url = format!("{}/checkouts", handle.config.environment.base_url());
        let response = reqwest::Client::new()
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", handle.config.bearer_token.as_str()),
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
        let text = response.text().await;
        serde_json::from_str(text.unwrap().as_str()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionsResponse {
    pub session_id: String,
    pub checkout_url: String,
}
