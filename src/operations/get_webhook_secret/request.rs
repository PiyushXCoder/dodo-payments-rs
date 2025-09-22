pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetWebhookSecret;

impl GetWebhookSecret {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetWebhookSecretConfig,
    ) -> Result<GetWebhookSecretResponse, crate::errors::Error> {
        let uri = format!("/webhooks/{}/secret", config.webhook_id);
        let response = make_reqwest(handle, Method::GET, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
