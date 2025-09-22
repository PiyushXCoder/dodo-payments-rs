use super::*;
use crate::{client::Handle, operations::update_webhook_headers::UpdateWebhookHeadersResponse};
use std::{collections::HashMap, sync::Arc};

pub struct UpdateWebhookHeadersBuilder {
    pub config: UpdateWebhookHeadersConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateWebhookHeadersBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String, headers: HashMap<String, String>) -> Self {
        Self {
            handle,
            config: UpdateWebhookHeadersConfig {
                webhook_id,
                headers,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.config.headers = headers;
        self
    }

    pub async fn send(self) -> Result<UpdateWebhookHeadersResponse, crate::errors::Error> {
        Ok(UpdateWebhookHeaders::orchestrate(self.handle, self.config).await?)
    }
}
