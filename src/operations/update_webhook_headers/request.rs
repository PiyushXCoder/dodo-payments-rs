pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UpdateWebhookHeaders;

impl UpdateWebhookHeaders {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateWebhookHeadersConfig,
    ) -> Result<UpdateWebhookHeadersResponse, crate::errors::Error> {
        let webhook_id = &config.webhook_id;
        let body = serde_json::to_string(&config).unwrap();
        let uri = format!("/webhooks/{}/headers", webhook_id);
        let response = make_reqwest(handle, Method::PATCH, &uri, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
