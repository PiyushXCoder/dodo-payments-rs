use super::*;
use crate::{client::Handle, operations::get_line_items::GetLineItemsResponse};

use std::sync::Arc;

pub struct GetLineItemsBuilder {
    pub config: GetLineItemsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetLineItemsBuilder {
    pub fn new(handle: Arc<Handle>, payment_id: String) -> Self {
        Self {
            handle,
            config: GetLineItemsConfig {
                payment_id,
            },
        }
    }

    pub fn payment_id(mut self, payment_id: String) -> Self {
        self.config.payment_id = payment_id;
        self
    }

    pub async fn send(self) -> Result<GetLineItemsResponse, crate::errors::Error> {
        Ok(GetLineItems::orchestrate(self.handle, self.config).await?)
    }
}
