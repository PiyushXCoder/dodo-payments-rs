use super::*;
use crate::{client::Handle, operations::get_brand::GetBrandResponse};

use std::sync::Arc;

pub struct GetBrandBuilder {
    pub(crate) handle: Arc<Handle>,
    pub brand_id: String,
}

impl GetBrandBuilder {
    pub fn new(handle: Arc<Handle>, brand_id: String) -> Self {
        Self {
            handle,
            brand_id,
        }
    }

    pub async fn send(self) -> Result<GetBrandResponse, crate::errors::Error> {
        Ok(GetBrand::orchestrate(self.handle, self.brand_id).await?)
    }
}