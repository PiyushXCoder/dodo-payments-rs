use super::*;
use crate::{client::Handle, operations::create_brand::CreateBrandResponse};

use std::sync::Arc;

pub struct CreateBrandBuilder {
    pub config: CreateBrandConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateBrandBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: CreateBrandConfig {
                name: None,
                description: None,
                statement_descriptor: None,
                support_email: None,
                url: None,
            },
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.config.statement_descriptor = Some(statement_descriptor);
        self
    }

    pub fn support_email(mut self, support_email: String) -> Self {
        self.config.support_email = Some(support_email);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.config.url = Some(url);
        self
    }

    pub async fn send(self) -> Result<CreateBrandResponse, crate::errors::Error> {
        Ok(CreateBrand::orchestrate(self.handle, self.config).await?)
    }
}