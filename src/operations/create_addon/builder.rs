use super::super::common::structs::{Currency, TaxCategory};
use super::*;
use crate::{client::Handle, operations::create_addon::CreateAddonResponse};

use std::sync::Arc;

pub struct CreateAddonBuilder {
    pub config: CreateAddonConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateAddonBuilder {
    pub fn new(
        handle: Arc<Handle>,
        name: String,
        tax_category: TaxCategory,
        price: i32,
        currency: Currency,
    ) -> Self {
        Self {
            handle,
            config: CreateAddonConfig {
                name,
                tax_category,
                price,
                currency,
                description: None,
            },
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    pub fn tax_category(mut self, tax_category: TaxCategory) -> Self {
        self.config.tax_category = tax_category;
        self
    }

    pub fn price(mut self, price: i32) -> Self {
        self.config.price = price;
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.config.currency = currency;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub async fn send(self) -> Result<CreateAddonResponse, crate::errors::Error> {
        Ok(CreateAddon::orchestrate(self.handle, self.config).await?)
    }
}