use super::super::common::structs::*;
use super::*;
use crate::{client::Handle, operations::checkout_sessions::CheckoutSessionsResponse};

use std::{collections::HashMap, sync::Arc};

pub struct CheckoutSessionsBuilder {
    pub config: CheckoutSessionsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CheckoutSessionsBuilder {
    pub fn new(handle: Arc<Handle>, product_cart: Vec<ProductItem>) -> Self {
        Self {
            handle,
            config: CheckoutSessionsConfig {
                product_cart,
                customer: None,
                billing_address: None,
                allowed_payment_method_types: None,
                billing_currency: None,
                show_saved_payment_methods: None,
                return_url: None,
                confirm: None,
                discount_code: None,
                metadata: None,
                customization: None,
                feature_flags: None,
                subscription_data: None,
            },
        }
    }

    pub fn product_cart(mut self, items: Vec<ProductItem>) -> Self {
        self.config.product_cart = items;
        self
    }

    pub fn customization(mut self, customization: CheckoutSessionCustomization) -> Self {
        self.config.customization = Some(customization);
        self
    }

    pub fn feature_flags(mut self, feature_flags: CheckoutSessionFlags) -> Self {
        self.config.feature_flags = Some(feature_flags);
        self
    }

    pub fn subscription_data(mut self, subscription_data: SubscriptionData) -> Self {
        self.config.subscription_data = Some(subscription_data);
        self
    }

    pub fn customer(mut self, customer: CustomerInfo) -> Self {
        self.config.customer = Some(customer);
        self
    }

    pub fn billing_address(mut self, address: BillingAddress) -> Self {
        self.config.billing_address = Some(address);
        self
    }

    pub fn allowed_payment_method_types(mut self, methods: Vec<String>) -> Self {
        self.config.allowed_payment_method_types = Some(methods);
        self
    }

    pub fn billing_currency(mut self, currency: String) -> Self {
        self.config.billing_currency = Some(currency);
        self
    }

    pub fn show_saved_payment_methods(mut self, show: bool) -> Self {
        self.config.show_saved_payment_methods = Some(show);
        self
    }

    pub fn return_url(mut self, url: String) -> Self {
        self.config.return_url = Some(url);
        self
    }

    pub fn confirm(mut self, confirm: bool) -> Self {
        self.config.confirm = Some(confirm);
        self
    }

    pub fn discount_code(mut self, code: String) -> Self {
        self.config.discount_code = Some(code);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.config.metadata = Some(metadata);
        self
    }

    pub async fn send(self) -> Result<CheckoutSessionsResponse, crate::errors::Error> {
        Ok(CheckoutSessions::orchestrate(self.handle, self.config).await?)
    }
}
