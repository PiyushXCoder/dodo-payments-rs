use super::super::common::structs::{Currency, TaxCategory};
use super::*;
use crate::{client::Handle, operations::update_addon::UpdateAddonResponse};

use std::sync::Arc;

pub struct UpdateAddonBuilder {
    pub addon_id: String,
    pub config: UpdateAddonConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateAddonBuilder {
    pub fn new(handle: Arc<Handle>, addon_id: String) -> Self {
        Self {
            handle,
            addon_id,
            config: UpdateAddonConfig {
                name: None,
                tax_category: None,
                price: None,
                currency: None,
                description: None,
                image_id: None,
            },
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = Some(name);
        self
    }

    pub fn tax_category(mut self, tax_category: TaxCategory) -> Self {
        self.config.tax_category = Some(tax_category);
        self
    }

    pub fn price(mut self, price: i32) -> Self {
        self.config.price = Some(price);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.config.currency = Some(currency);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub fn image_id(mut self, image_id: String) -> Self {
        self.config.image_id = Some(image_id);
        self
    }

    pub async fn send(self) -> Result<UpdateAddonResponse, crate::errors::Error> {
        Ok(UpdateAddon::orchestrate(self.handle, self.addon_id, self.config).await?)
    }
}