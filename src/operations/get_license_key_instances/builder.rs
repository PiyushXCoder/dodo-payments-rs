use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct GetLicenseKeyInstancesBuilder {
    pub config: GetLicenseKeyInstancesConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetLicenseKeyInstancesBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: GetLicenseKeyInstancesConfig {
                page_size: None,
                page_number: None,
                license_key_id: None,
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

    pub fn license_key_id(mut self, license_key_id: String) -> Self {
        self.config.license_key_id = Some(license_key_id);
        self
    }

    pub async fn send(self) -> Result<ListLicenseKeyInstancesResponse, crate::errors::Error> {
        Ok(GetLicenseKeyInstances::orchestrate(self.handle, self.config).await?)
    }
}
