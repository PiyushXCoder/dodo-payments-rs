use super::*;
use crate::{client::Handle, operations::patch_customer::PatchCustomerResponse};

use std::sync::Arc;

pub struct PatchCustomerBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
    pub config: PatchCustomerConfig,
}

impl PatchCustomerBuilder {
    pub fn new(handle: Arc<Handle>, customer_id: String) -> Self {
        Self {
            handle,
            customer_id,
            config: PatchCustomerConfig {
                name: None,
                phone_number: None,
            },
        }
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        self.config.name = name;
        self
    }

    pub fn phone_number(mut self, phone_number: Option<String>) -> Self {
        self.config.phone_number = phone_number;
        self
    }

    pub async fn send(self) -> Result<PatchCustomerResponse, crate::errors::Error> {
        Ok(PatchCustomer::orchestrate(self.handle, self.customer_id, self.config).await?)
    }
}
