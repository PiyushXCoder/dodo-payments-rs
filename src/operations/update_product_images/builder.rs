use super::*;
use crate::{client::Handle, operations::update_product_images::UpdateProductImageResponse};
use std::sync::Arc;

pub struct UpdateProductImagesBuilder {
    pub config: UpdateProductImagesConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateProductImagesBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UpdateProductImagesConfig {
                id,
                force_update: None,
            },
        }
    }

    pub fn force_update(mut self, force_update: bool) -> Self {
        self.config.force_update = Some(force_update);
        self
    }

    pub async fn send(self) -> Result<UpdateProductImageResponse, crate::errors::Error> {
        Ok(UpdateProductImages::orchestrate(self.handle, self.config).await?)
    }
}
