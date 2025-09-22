use super::*;
use crate::{client::Handle, operations::list_addons::ListAddonsResponse};

use std::sync::Arc;

pub struct ListAddonsBuilder {
    pub config: ListAddonsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListAddonsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListAddonsConfig {
                page_size: None,
                page_number: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: u32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub async fn send(self) -> Result<ListAddonsResponse, crate::errors::Error> {
        Ok(ListAddons::orchestrate(self.handle, self.config).await?)
    }
}