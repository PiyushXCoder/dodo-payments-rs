use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct ActivateLicenseBuilder {
    pub config: ActivateLicenseConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ActivateLicenseBuilder {
    pub fn new(handle: Arc<Handle>, license_key: String, name: String) -> Self {
        Self {
            handle,
            config: ActivateLicenseConfig {
                license_key,
                name,
            },
        }
    }

    pub fn license_key(mut self, license_key: String) -> Self {
        self.config.license_key = license_key;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    pub async fn send(self) -> Result<ActivateLicenseResponse, crate::errors::Error> {
        Ok(ActivateLicense::orchestrate(self.handle, self.config).await?)
    }
}
