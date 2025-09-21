use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct UpdateLicenseKeyInstanceBuilder {
    pub config: UpdateLicenseKeyInstanceConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateLicenseKeyInstanceBuilder {
    pub fn new(handle: Arc<Handle>, id: String, name: String) -> Self {
        Self {
            handle,
            config: UpdateLicenseKeyInstanceConfig {
                id,
                name,
            },
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.config.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    pub async fn send(self) -> Result<UpdateLicenseKeyInstanceResponse, crate::errors::Error> {
        Ok(UpdateLicenseKeyInstance::orchestrate(self.handle, self.config).await?)
    }
}
