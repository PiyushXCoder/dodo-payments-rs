use super::*;
use crate::{client::Handle, operations::get_webhook_secret::GetWebhookSecretResponse};

use std::sync::Arc;

pub struct GetWebhookSecretBuilder {
    pub config: GetWebhookSecretConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetWebhookSecretBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String) -> Self {
        Self {
            handle,
            config: GetWebhookSecretConfig {
                webhook_id,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub async fn send(self) -> Result<GetWebhookSecretResponse, crate::errors::Error> {
        Ok(GetWebhookSecret::orchestrate(self.handle, self.config).await?)
    }
}
