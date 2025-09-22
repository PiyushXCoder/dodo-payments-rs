use super::*;
use crate::{client::Handle, operations::list_brands::ListBrandsResponse};

use std::sync::Arc;

pub struct ListBrandsBuilder {
    pub(crate) handle: Arc<Handle>,
}

impl ListBrandsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
        }
    }

    pub async fn send(self) -> Result<ListBrandsResponse, crate::errors::Error> {
        Ok(ListBrands::orchestrate(self.handle).await?)
    }
}