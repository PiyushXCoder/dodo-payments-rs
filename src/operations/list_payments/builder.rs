use chrono::{DateTime, Utc};

use super::*;
use crate::client::Handle;

use std::sync::Arc;

pub struct ListPaymentsBuilder {
    pub config: ListPaymentsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListPaymentsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListPaymentsConfig {
                created_at_gte: None,
                created_at_lte: None,
                page_size: None,
                page_number: None,
                customer_id: None,
                subscription_id: None,
                status: None,
                brand_id: None,
            },
        }
    }

    pub fn created_at_gte(mut self, created_at_gte: DateTime<Utc>) -> Self {
        self.config.created_at_gte = Some(created_at_gte);
        self
    }

    pub fn created_at_lte(mut self, created_at_lte: DateTime<Utc>) -> Self {
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

    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.config.customer_id = Some(customer_id.into());
        self
    }
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.config.subscription_id = Some(subscription_id.into());
        self
    }
    pub fn status(mut self, status: Status) -> Self {
        self.config.status = Some(status);
        self
    }
    pub fn brand_id(mut self, brand_id: impl Into<String>) -> Self {
        self.config.brand_id = Some(brand_id.into());
        self
    }

    pub async fn send(self) -> Result<ListPaymentsResponse, crate::errors::Error> {
        Ok(ListPayments::orchestrate(self.handle, self.config).await?)
    }
}
