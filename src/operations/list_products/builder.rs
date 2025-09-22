use super::*;
use crate::client::Handle;
use super::response::GetProductsListResponse;
use std::sync::Arc;

pub struct ListProductsBuilder {
    pub config: ListProductsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListProductsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListProductsConfig {
                page_size: None,
                page_number: None,
                archived: None,
                recurring: None,
                brand_id: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: i64) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: i64) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.config.archived = Some(archived);
        self
    }

    pub fn recurring(mut self, recurring: bool) -> Self {
        self.config.recurring = Some(recurring);
        self
    }

    pub fn brand_id(mut self, brand_id: String) -> Self {
        self.config.brand_id = Some(brand_id);
        self
    }

    pub async fn send(self) -> Result<GetProductsListResponse, crate::errors::Error> {
        Ok(ListProducts::orchestrate(self.handle, self.config).await?)
    }
}
