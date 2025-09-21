use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct GetLicenseKeysBuilder {
    pub config: GetLicenseKeysConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetLicenseKeysBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: GetLicenseKeysConfig {
                page_size: None,
                page_number: None,
                customer_id: None,
                status: None,
                product_id: None,
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

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.config.customer_id = Some(customer_id);
        self
    }

    pub fn status(mut self, status: LicenseKeyStatus) -> Self {
        self.config.status = Some(status);
        self
    }

    pub fn product_id(mut self, product_id: String) -> Self {
        self.config.product_id = Some(product_id);
        self
    }

    pub async fn send(self) -> Result<ListLicenseKeysResponse, crate::errors::Error> {
        Ok(GetLicenseKeys::orchestrate(self.handle, self.config).await?)
    }
}
