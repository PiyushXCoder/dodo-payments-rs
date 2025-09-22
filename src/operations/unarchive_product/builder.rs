use super::*;
use crate::{client::Handle, operations::unarchive_product::UnarchiveProductResponse};
use std::sync::Arc;

pub struct UnarchiveProductBuilder {
    pub config: UnarchiveProductConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UnarchiveProductBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UnarchiveProductConfig {
                id,
            },
        }
    }

    pub async fn send(self) -> Result<UnarchiveProductResponse, crate::errors::Error> {
        Ok(UnarchiveProduct::orchestrate(self.handle, self.config).await?)
    }
}
