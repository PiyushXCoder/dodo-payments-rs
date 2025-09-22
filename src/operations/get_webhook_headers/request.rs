pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetWebhookHeaders;

impl GetWebhookHeaders {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetWebhookHeadersConfig,
    ) -> Result<GetWebhookHeadersResponse, crate::errors::Error> {
        let uri = format!("/webhooks/{}/headers", config.webhook_id);
        let response = make_reqwest(handle, Method::GET, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
