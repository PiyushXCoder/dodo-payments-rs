use super::*;
use crate::{client::Handle, operations::list_webhooks::ListWebhooksResponse};
use std::sync::Arc;

pub struct ListWebhooksBuilder {
    pub config: ListWebhooksConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListWebhooksBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListWebhooksConfig {
                limit: None,
                iterator: None,
            },
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.config.limit = Some(limit);
        self
    }

    pub fn iterator(mut self, iterator: String) -> Self {
        self.config.iterator = Some(iterator);
        self
    }

    pub async fn send(self) -> Result<ListWebhooksResponse, crate::errors::Error> {
        Ok(ListWebhooks::orchestrate(self.handle, self.config).await?)
    }
}