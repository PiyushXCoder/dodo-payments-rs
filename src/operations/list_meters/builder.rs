use super::*;
use crate::{client::Handle, operations::list_meters::ListMetersResponse};

use std::sync::Arc;

pub struct ListMetersBuilder {
    pub config: ListMetersConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListMetersBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListMetersConfig {
                page_size: None,
                page_number: None,
                archived: None,
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

    pub fn archived(mut self, archived: bool) -> Self {
        self.config.archived = Some(archived);
        self
    }

    pub async fn send(self) -> Result<ListMetersResponse, crate::errors::Error> {
        Ok(ListMeters::orchestrate(self.handle, self.config).await?)
    }
}