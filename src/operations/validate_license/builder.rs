use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct ValidateLicenseBuilder {
    pub config: ValidateLicenseConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ValidateLicenseBuilder {
    pub fn new(handle: Arc<Handle>, license_key: String) -> Self {
        Self {
            handle,
            config: ValidateLicenseConfig {
                license_key,
                license_key_instance_id: None,
            },
        }
    }

    pub fn license_key(mut self, license_key: String) -> Self {
        self.config.license_key = license_key;
        self
    }

    pub fn license_key_instance_id(mut self, license_key_instance_id: String) -> Self {
        self.config.license_key_instance_id = Some(license_key_instance_id);
        self
    }

    pub async fn send(self) -> Result<ValidateLicenseResponse, crate::errors::Error> {
        Ok(ValidateLicense::orchestrate(self.handle, self.config).await?)
    }
}
