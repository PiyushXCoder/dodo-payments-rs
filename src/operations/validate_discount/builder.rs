use super::*;
use crate::{client::Handle, operations::validate_discount::ValidateDiscountResponse};

use std::sync::Arc;

pub struct ValidateDiscountBuilder {
    pub config: ValidateDiscountConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ValidateDiscountBuilder {
    pub fn new(handle: Arc<Handle>, discount_id: String) -> Self {
        Self {
            handle,
            config: ValidateDiscountConfig {
                discount_id,
            },
        }
    }

    pub async fn send(self) -> Result<ValidateDiscountResponse, crate::errors::Error> {
        Ok(ValidateDiscount::orchestrate(self.handle, self.config).await?)
    }
}
