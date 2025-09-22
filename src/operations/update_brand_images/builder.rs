use super::*;
use crate::{client::Handle, operations::update_brand_images::UpdateBrandImageResponse};
use std::sync::Arc;

pub struct UpdateBrandImagesBuilder {
    pub config: UpdateBrandImagesConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateBrandImagesBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UpdateBrandImagesConfig {
                id,
            },
        }
    }

    pub async fn send(self) -> Result<UpdateBrandImageResponse, crate::errors::Error> {
        Ok(UpdateBrandImages::orchestrate(self.handle, self.config).await?)
    }
}