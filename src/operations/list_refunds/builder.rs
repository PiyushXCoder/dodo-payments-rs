use super::*;
use crate::{client::Handle, operations::list_refunds::ListRefundsResponse};

use std::sync::Arc;

pub struct ListRefundsBuilder {
    pub config: ListRefundsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListRefundsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListRefundsConfig {
                created_at_gte: None,
                created_at_lte: None,
                page_size: None,
                page_number: None,
                status: None,
                customer_id: None,
            },
        }
    }

    pub fn created_at_gte(mut self, created_at_gte: String) -> Self {
        self.config.created_at_gte = Some(created_at_gte);
        self
    }

    pub fn created_at_lte(mut self, created_at_lte: String) -> Self {
        self.config.created_at_lte = Some(created_at_lte);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: u32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub fn status(mut self, status: String) -> Self {
        self.config.status = Some(status);
        self
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.config.customer_id = Some(customer_id);
        self
    }

    pub async fn send(self) -> Result<ListRefundsResponse, crate::errors::Error> {
        Ok(ListRefunds::orchestrate(self.handle, self.config).await?)
    }
}