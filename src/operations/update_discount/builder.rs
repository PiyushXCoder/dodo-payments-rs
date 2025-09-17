use super::*;
use crate::{client::Handle, operations::common::structs::DiscountType, operations::update_discount::UpdateDiscountResponse};

use std::sync::Arc;

pub struct UpdateDiscountBuilder {
    pub config: UpdateDiscountConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateDiscountBuilder {
    pub fn new(handle: Arc<Handle>, discount_id: String) -> Self {
        Self {
            handle,
            config: UpdateDiscountConfig {
                discount_id,
                amount: None,
                code: None,
                expires_at: None,
                name: None,
                restricted_to: None,
                subscription_cycles: None,
                discount_type: None,
                usage_limit: None,
            },
        }
    }

    pub fn amount(mut self, amount: i32) -> Self {
        self.config.amount = Some(amount);
        self
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

    pub fn discount_type(mut self, discount_type: DiscountType) -> Self {
        self.config.discount_type = Some(discount_type);
        self
    }

    pub fn usage_limit(mut self, usage_limit: i32) -> Self {
        self.config.usage_limit = Some(usage_limit);
        self
    }

    pub async fn send(self) -> Result<UpdateDiscountResponse, crate::errors::Error> {
        Ok(UpdateDiscount::orchestrate(self.handle, self.config).await?)
    }
}
