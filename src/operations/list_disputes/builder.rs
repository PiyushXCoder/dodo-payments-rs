use super::*;
use crate::{client::Handle, operations::list_disputes::ListDisputesResponse};

use std::sync::Arc;

pub struct ListDisputesBuilder {
    pub config: ListDisputesConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListDisputesBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListDisputesConfig {
                created_at_gte: None,
                created_at_lte: None,
                page_size: None,
                page_number: None,
                dispute_status: None,
                dispute_stage: None,
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

    pub fn dispute_status(mut self, dispute_status: String) -> Self {
        self.config.dispute_status = Some(dispute_status);
        self
    }

    pub fn dispute_stage(mut self, dispute_stage: String) -> Self {
        self.config.dispute_stage = Some(dispute_stage);
        self
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.config.customer_id = Some(customer_id);
        self
    }

    pub async fn send(self) -> Result<ListDisputesResponse, crate::errors::Error> {
        Ok(ListDisputes::orchestrate(self.handle, self.config).await?)
    }
}