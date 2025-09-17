use super::*;
use crate::{client::Handle, operations::delete_discount::DeleteDiscountResponse};

use std::sync::Arc;

pub struct DeleteDiscountBuilder {
    pub config: DeleteDiscountConfig,
    pub(crate) handle: Arc<Handle>,
}

impl DeleteDiscountBuilder {
    pub fn new(handle: Arc<Handle>, discount_id: String) -> Self {
        Self {
            handle,
            config: DeleteDiscountConfig {
                discount_id,
            },
        }
    }

    pub async fn send(self) -> Result<DeleteDiscountResponse, crate::errors::Error> {
        Ok(DeleteDiscount::orchestrate(self.handle, self.config).await?)
    }
}
