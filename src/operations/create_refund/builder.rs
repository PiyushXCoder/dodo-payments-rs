use super::super::common::structs::PartialRefundItem;
use super::*;
use crate::{client::Handle, operations::create_refund::CreateRefundResponse};

use std::sync::Arc;

pub struct CreateRefundBuilder {
    pub config: CreateRefundConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateRefundBuilder {
    pub fn new(handle: Arc<Handle>, payment_id: String) -> Self {
        Self {
            handle,
            config: CreateRefundConfig {
                payment_id,
                items: None,
                reason: None,
            },
        }
    }

    pub fn payment_id(mut self, payment_id: String) -> Self {
        self.config.payment_id = payment_id;
        self
    }

    pub fn items(mut self, items: Vec<PartialRefundItem>) -> Self {
        self.config.items = Some(items);
        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.config.reason = Some(reason);
        self
    }

    pub async fn send(self) -> Result<CreateRefundResponse, crate::errors::Error> {
        Ok(CreateRefund::orchestrate(self.handle, self.config).await?)
    }
}