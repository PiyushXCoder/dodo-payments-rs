use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct DeactivateLicenseBuilder {
    pub config: DeactivateLicenseConfig,
    pub(crate) handle: Arc<Handle>,
}

impl DeactivateLicenseBuilder {
    pub fn new(handle: Arc<Handle>, license_key: String, license_key_instance_id: String) -> Self {
        Self {
            handle,
            config: DeactivateLicenseConfig {
                license_key,
                license_key_instance_id,
            },
        }
    }

    pub fn license_key(mut self, license_key: String) -> Self {
        self.config.license_key = license_key;
        self
    }

    pub fn license_key_instance_id(mut self, license_key_instance_id: String) -> Self {
        self.config.license_key_instance_id = license_key_instance_id;
        self
    }

    pub async fn send(self) -> Result<DeactivateLicenseResponse, crate::errors::Error> {
        Ok(DeactivateLicense::orchestrate(self.handle, self.config).await?)
    }
}
