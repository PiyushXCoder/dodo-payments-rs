use super::*;
use crate::{client::Handle, operations::update_addon_images::UpdateAddonImagesResponse};

use std::sync::Arc;

pub struct UpdateAddonImagesBuilder {
    pub addon_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateAddonImagesBuilder {
    pub fn new(handle: Arc<Handle>, addon_id: String) -> Self {
        Self {
            handle,
            addon_id,
        }
    }

    pub async fn send(self) -> Result<UpdateAddonImagesResponse, crate::errors::Error> {
        Ok(UpdateAddonImages::orchestrate(self.handle, self.addon_id).await?)
    }
}