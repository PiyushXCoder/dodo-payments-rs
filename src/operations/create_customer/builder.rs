use super::*;
use crate::{client::Handle, operations::create_customer::CreateCustomerResponse};

use std::sync::Arc;

pub struct CreateCustomerBuilder {
    pub config: CreateCustomerConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateCustomerBuilder {
    pub fn new(handle: Arc<Handle>, email: String, name: String) -> Self {
        Self {
            handle,
            config: CreateCustomerConfig {
                email,
                name,
                phone_number: None,
            },
        }
    }

    pub fn email(mut self, email: String) -> Self {
        self.config.email = email;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    pub fn phone_number(mut self, phone_number: Option<String>) -> Self {
        self.config.phone_number = phone_number;
        self
    }

    pub async fn send(self) -> Result<CreateCustomerResponse, crate::errors::Error> {
        Ok(CreateCustomer::orchestrate(self.handle, self.config).await?)
    }
}
