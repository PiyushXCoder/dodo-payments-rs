use super::*;
use crate::{client::Handle, operations::create_product::GetProductResponse};
use std::sync::Arc;

pub struct GetProductBuilder {
    pub config: GetProductConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetProductBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: GetProductConfig {
                id,
            },
        }
    }

    pub async fn send(self) -> Result<GetProductResponse, crate::errors::Error> {
        Ok(GetProduct::orchestrate(self.handle, self.config).await?)
    }
}
