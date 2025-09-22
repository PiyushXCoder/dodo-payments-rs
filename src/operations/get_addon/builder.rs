use super::*;
use crate::{client::Handle, operations::get_addon::GetAddonResponse};

use std::sync::Arc;

pub struct GetAddonBuilder {
    pub addon_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetAddonBuilder {
    pub fn new(handle: Arc<Handle>, addon_id: String) -> Self {
        Self {
            handle,
            addon_id,
        }
    }

    pub async fn send(self) -> Result<GetAddonResponse, crate::errors::Error> {
        Ok(GetAddon::orchestrate(self.handle, self.addon_id).await?)
    }
}