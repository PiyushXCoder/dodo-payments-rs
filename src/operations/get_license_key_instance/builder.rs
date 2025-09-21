use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct GetLicenseKeyInstanceBuilder {
    pub config: GetLicenseKeyInstanceConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetLicenseKeyInstanceBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: GetLicenseKeyInstanceConfig {
                id,
            },
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.config.id = id;
        self
    }

    pub async fn send(self) -> Result<GetLicenseKeyInstanceResponse, crate::errors::Error> {
        Ok(GetLicenseKeyInstance::orchestrate(self.handle, self.config).await?)
    }
}
