use super::*;
use crate::{client::Handle, operations::list_payouts::ListPayoutsResponse};

use std::sync::Arc;

pub struct ListPayoutsBuilder {
    pub config: ListPayoutsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListPayoutsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListPayoutsConfig {
                page_size: None,
                page_number: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: u32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub async fn send(self) -> Result<ListPayoutsResponse, crate::errors::Error> {
        Ok(ListPayouts::orchestrate(self.handle, self.config).await?)
    }
}