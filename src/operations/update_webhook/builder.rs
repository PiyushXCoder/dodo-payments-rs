use super::*;
use crate::{client::Handle, operations::update_webhook::UpdateWebhookResponse};
use std::{collections::HashMap, sync::Arc};
use super::super::common::structs::EventType;

pub struct UpdateWebhookBuilder {
    pub config: UpdateWebhookConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateWebhookBuilder {
    pub fn new(handle: Arc<Handle>, webhook_id: String) -> Self {
        Self {
            handle,
            config: UpdateWebhookConfig {
                webhook_id,
                description: None,
                disabled: None,
                filter_types: None,
                metadata: None,
                rate_limit: None,
                url: None,
            },
        }
    }

    pub fn webhook_id(mut self, webhook_id: String) -> Self {
        self.config.webhook_id = webhook_id;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = Some(disabled);
        self
    }

    pub fn filter_types(mut self, filter_types: Vec<EventType>) -> Self {
        self.config.filter_types = Some(filter_types);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.config.metadata = Some(metadata);
        self
    }

    pub fn rate_limit(mut self, rate_limit: i32) -> Self {
        self.config.rate_limit = Some(rate_limit);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.config.url = Some(url);
        self
    }

    pub async fn send(self) -> Result<UpdateWebhookResponse, crate::errors::Error> {
        Ok(UpdateWebhook::orchestrate(self.handle, self.config).await?)
    }
}
