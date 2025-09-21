use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct GetLicenseKeyBuilder {
    pub config: GetLicenseKeyConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetLicenseKeyBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: GetLicenseKeyConfig {
                id,
            },
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.config.id = id;
        self
    }

    pub async fn send(self) -> Result<GetLicenseKeyResponse, crate::errors::Error> {
        Ok(GetLicenseKey::orchestrate(self.handle, self.config).await?)
    }
}
