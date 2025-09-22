use super::*;
use crate::{client::Handle, operations::get_webhook::GetWebhookResponse};

use std::sync::Arc;

pub struct GetWebhookBuilder {
    pub config: GetWebhookConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetWebhookBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String) -> Self {
        Self {
            handle,
            config: GetWebhookConfig {
                webhook_id,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub async fn send(self) -> Result<GetWebhookResponse, crate::errors::Error> {
        Ok(GetWebhook::orchestrate(self.handle, self.config).await?)
    }
}
