pub use super::config::*;
use super::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct ChangePlan;

impl ChangePlan {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        subscription_id: String,
        config: ChangePlanConfig,
    ) -> Result<ChangePlanResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let url = format!("/subscriptions/{}/change-plan", subscription_id);
        let response = make_reqwest(handle, Method::POST, &url, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
