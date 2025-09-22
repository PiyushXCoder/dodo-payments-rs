pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct DeleteWebhook;

impl DeleteWebhook {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: DeleteWebhookConfig,
    ) -> Result<DeleteWebhookResponse, crate::errors::Error> {
        let uri = format!("/webhooks/{}", config.webhook_id);
        let response = make_reqwest(handle, Method::DELETE, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
