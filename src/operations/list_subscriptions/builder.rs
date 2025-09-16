use super::{
    ListSubscriptions, ListSubscriptionsConfig, ListSubscriptionsResponse, SubscriptionStatus,
};
use crate::{client::Handle, errors::Error};
use std::sync::Arc;

pub struct ListSubscriptionsBuilder {
    pub config: ListSubscriptionsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListSubscriptionsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListSubscriptionsConfig {
                created_at_gte: None,
                created_at_lte: None,
                page_size: None,
                page_number: None,
                customer_id: None,
                status: None,
                brand_id: None,
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

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.config.customer_id = Some(customer_id);
        self
    }

    pub fn status(mut self, status: SubscriptionStatus) -> Self {
        self.config.status = Some(status);
        self
    }

    pub fn brand_id(mut self, brand_id: String) -> Self {
        self.config.brand_id = Some(brand_id);
        self
    }

    pub async fn send(self) -> Result<ListSubscriptionsResponse, Error> {
        ListSubscriptions::orchestrate(self.handle, self.config).await
    }
}
