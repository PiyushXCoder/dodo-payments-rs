use super::*;
use crate::{client::Handle, operations::common::structs::DiscountType, operations::create_discount::CreateDiscountResponse};

use std::sync::Arc;

pub struct CreateDiscountBuilder {
    pub config: CreateDiscountConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateDiscountBuilder {
    pub fn new(handle: Arc<Handle>, amount: i32, discount_type: DiscountType) -> Self {
        Self {
            handle,
            config: CreateDiscountConfig {
                amount,
                discount_type,
                code: None,
                expires_at: None,
                name: None,
                restricted_to: None,
                subscription_cycles: None,
                usage_limit: None,
            },
        }
    }

    pub fn code(mut self, code: String) -> Self {
        self.config.code = Some(code);
        self
    }

    pub fn expires_at(mut self, expires_at: String) -> Self {
        self.config.expires_at = Some(expires_at);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = Some(name);
        self
    }

    pub fn restricted_to(mut self, restricted_to: Vec<String>) -> Self {
        self.config.restricted_to = Some(restricted_to);
        self
    }

    pub fn subscription_cycles(mut self, subscription_cycles: i32) -> Self {
        self.config.subscription_cycles = Some(subscription_cycles);
        self
    }

    pub fn usage_limit(mut self, usage_limit: i32) -> Self {
        self.config.usage_limit = Some(usage_limit);
        self
    }

    pub async fn send(self) -> Result<CreateDiscountResponse, crate::errors::Error> {
        Ok(CreateDiscount::orchestrate(self.handle, self.config).await?)
    }
}
