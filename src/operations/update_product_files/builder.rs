use super::*;
use crate::{client::Handle, operations::update_product_files::UploadProductFileResponse};
use std::sync::Arc;

pub struct UpdateProductFilesBuilder {
    pub config: UpdateProductFilesConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateProductFilesBuilder {
    pub fn new(handle: Arc<Handle>, id: String, file_name: String) -> Self {
        Self {
            handle,
            config: UpdateProductFilesConfig {
                id,
                body: UploadProductFile {
                    file_name,
                },
            },
        }
    }

    pub async fn send(self) -> Result<UploadProductFileResponse, crate::errors::Error> {
        Ok(UpdateProductFiles::orchestrate(self.handle, self.config).await?)
    }
}
