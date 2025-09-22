use super::*;
use crate::{client::Handle, operations::get_webhook_headers::GetWebhookHeadersResponse};

use std::sync::Arc;

pub struct GetWebhookHeadersBuilder {
    pub config: GetWebhookHeadersConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetWebhookHeadersBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String) -> Self {
        Self {
            handle,
            config: GetWebhookHeadersConfig {
                webhook_id,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub async fn send(self) -> Result<GetWebhookHeadersResponse, crate::errors::Error> {
        Ok(GetWebhookHeaders::orchestrate(self.handle, self.config).await?)
    }
}
