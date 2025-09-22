use super::*;
use crate::{client::Handle, operations::delete_webhook::DeleteWebhookResponse};

use std::sync::Arc;

pub struct DeleteWebhookBuilder {
    pub config: DeleteWebhookConfig,
    pub(crate) handle: Arc<Handle>,
}

impl DeleteWebhookBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String) -> Self {
        Self {
            handle,
            config: DeleteWebhookConfig {
                webhook_id,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub async fn send(self) -> Result<DeleteWebhookResponse, crate::errors::Error> {
        Ok(DeleteWebhook::orchestrate(self.handle, self.config).await?)
    }
}
