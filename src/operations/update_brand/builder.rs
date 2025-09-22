use super::*;
use crate::{client::Handle, operations::update_brand::UpdateBrandResponse};
use std::sync::Arc;

pub struct UpdateBrandBuilder {
    pub config: UpdateBrandConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateBrandBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UpdateBrandConfig {
                id,
                body: PatchBrandRequest {
                    image_id: None,
                    name: None,
                    statement_descriptor: None,
                    support_email: None,
                },
            },
        }
    }

    pub fn image_id(mut self, image_id: String) -> Self {
        self.config.body.image_id = Some(image_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.body.name = Some(name);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.config.body.statement_descriptor = Some(statement_descriptor);
        self
    }

    pub fn support_email(mut self, support_email: String) -> Self {
        self.config.body.support_email = Some(support_email);
        self
    }

    pub async fn send(self) -> Result<UpdateBrandResponse, crate::errors::Error> {
        Ok(UpdateBrand::orchestrate(self.handle, self.config).await?)
    }
}