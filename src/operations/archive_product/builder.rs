use super::*;
use crate::{client::Handle, operations::archive_product::ArchiveProductResponse};
use std::sync::Arc;

pub struct ArchiveProductBuilder {
    pub config: ArchiveProductConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ArchiveProductBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: ArchiveProductConfig {
                id,
            },
        }
    }

    pub async fn send(self) -> Result<ArchiveProductResponse, crate::errors::Error> {
        Ok(ArchiveProduct::orchestrate(self.handle, self.config).await?)
    }
}
