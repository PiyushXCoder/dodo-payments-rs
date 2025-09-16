pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CreateCharge;

impl CreateCharge {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CreateChargeConfig,
    ) -> Result<CreateChargeResponse, crate::errors::Error> {
        let subscription_id = config.subscription_id.clone();
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(
            handle,
            Method::POST,
            &format!("/subscriptions/{}/charge", subscription_id),
            Some(body),
        )
        .await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
