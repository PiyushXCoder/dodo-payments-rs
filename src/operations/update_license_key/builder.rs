use super::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct UpdateLicenseKeyBuilder {
    pub config: UpdateLicenseKeyConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateLicenseKeyBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UpdateLicenseKeyConfig {
                id,
                activations_limit: None,
                disabled: None,
                expires_at: None,
            },
        }
    }

    pub fn activations_limit(mut self, activations_limit: Option<i32>) -> Self {
        self.config.activations_limit = Some(activations_limit);
        self
    }

    pub fn disabled(mut self, disabled: Option<bool>) -> Self {
        self.config.disabled = Some(disabled);
        self
    }

    pub fn expires_at(mut self, expires_at: Option<String>) -> Self {
        self.config.expires_at = Some(expires_at);
        self
    }

    pub async fn send(self) -> Result<UpdateLicenseKeyResponse, crate::errors::Error> {
        Ok(UpdateLicenseKey::orchestrate(self.handle, self.config).await?)
    }
}
