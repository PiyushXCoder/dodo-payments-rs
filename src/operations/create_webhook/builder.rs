use super::*;
use crate::{client::Handle, operations::create_webhook::CreateWebhookResponse};
use std::{collections::HashMap, sync::Arc};
use super::super::common::structs::EventType;

pub struct CreateWebhookBuilder {
    pub config: CreateWebhookConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateWebhookBuilder {
    pub fn new(handle: Arc<Handle>, url: String) -> Self {
        Self {
            handle,
            config: CreateWebhookConfig {
                url,
                description: None,
                disabled: None,
                filter_types: None,
                headers: None,
                idempotency_key: None,
                metadata: None,
                rate_limit: None,
            },
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.config.url = url;
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

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.config.headers = Some(headers);
        self
    }

    pub fn idempotency_key(mut self, idempotency_key: String) -> Self {
        self.config.idempotency_key = Some(idempotency_key);
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

    pub async fn send(self) -> Result<CreateWebhookResponse, crate::errors::Error> {
        Ok(CreateWebhook::orchestrate(self.handle, self.config).await?)
    }
}
