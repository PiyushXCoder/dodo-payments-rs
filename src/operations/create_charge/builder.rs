use super::super::common::structs::{Currency, Metadata};
use super::{CreateChargeConfig, CreateChargeResponse, CustomerBalanceConfig};
use crate::{client::Handle, operations::create_charge::CreateCharge};

use std::sync::Arc;

pub struct CreateChargeBuilder {
    pub config: CreateChargeConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateChargeBuilder {
    pub fn new(handle: Arc<Handle>, subscription_id: String, product_price: i32) -> Self {
        Self {
            handle,
            config: CreateChargeConfig {
                subscription_id,
                product_price,
                adaptive_currency_fees_inclusive: None,
                customer_balance_config: None,
                metadata: None,
                product_currency: None,
                product_description: None,
            },
        }
    }

    pub fn adaptive_currency_fees_inclusive(mut self, inclusive: bool) -> Self {
        self.config.adaptive_currency_fees_inclusive = Some(inclusive);
        self
    }

    pub fn customer_balance_config(mut self, config: CustomerBalanceConfig) -> Self {
        self.config.customer_balance_config = Some(config);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.config.metadata = Some(metadata);
        self
    }

    pub fn product_currency(mut self, currency: Currency) -> Self {
        self.config.product_currency = Some(currency);
        self
    }

    pub fn product_description(mut self, description: String) -> Self {
        self.config.product_description = Some(description);
        self
    }

    pub async fn send(self) -> Result<CreateChargeResponse, crate::errors::Error> {
        Ok(CreateCharge::orchestrate(self.handle, self.config).await?)
    }
}
