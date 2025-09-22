use super::*;
use crate::{client::Handle, operations::list_customers::ListCustomersResponse};

use std::sync::Arc;

pub struct ListCustomersBuilder {
    pub config: ListCustomersConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListCustomersBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListCustomersConfig {
                page_size: None,
                page_number: None,
                email: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: i32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.config.email = Some(email);
        self
    }

    pub async fn send(self) -> Result<ListCustomersResponse, crate::errors::Error> {
        Ok(ListCustomers::orchestrate(self.handle, self.config).await?)
    }
}
