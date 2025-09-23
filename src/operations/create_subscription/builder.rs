use super::super::common::structs::*;
use super::*;
use crate::{
    client::Handle, operations::create_subscription::response::CreateSubscriptionResponse,
};
use std::{collections::HashMap, sync::Arc};

pub struct CreateSubscriptionBuilder {
    pub config: CreateSubscriptionConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateSubscriptionBuilder {
    pub fn new(
        handle: Arc<Handle>,
        product_id: String,
        quantity: i32,
        customer: CustomerRequest,
        billing: BillingAddress,
    ) -> Self {
        Self {
            handle,
            config: CreateSubscriptionConfig {
                product_id,
                quantity,
                customer,
                billing,
                addons: None,
                allowed_payment_method_types: None,
                billing_currency: None,
                discount_code: None,
                metadata: None,
                on_demand: None,
                payment_link: None,
                return_url: None,
                show_saved_payment_methods: None,
                tax_id: None,
                trial_period_days: None,
            },
        }
    }

    pub fn addons(mut self, addons: Vec<Addon>) -> Self {
        self.config.addons = Some(addons);
        self
    }

    pub fn allowed_payment_method_types(mut self, methods: Vec<PaymentMethodTypes>) -> Self {
        self.config.allowed_payment_method_types = Some(methods);
        self
    }

    pub fn billing_currency(mut self, currency: Currency) -> Self {
        self.config.billing_currency = Some(currency);
        self
    }

    pub fn discount_code(mut self, code: String) -> Self {
        self.config.discount_code = Some(code);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.config.metadata = Some(RequestMetadata(metadata));
        self
    }

    pub fn on_demand(mut self, on_demand: OnDemandSubscriptionReq) -> Self {
        self.config.on_demand = Some(on_demand);
        self
    }

    pub fn payment_link(mut self, payment_link: bool) -> Self {
        self.config.payment_link = Some(payment_link);
        self
    }

    pub fn return_url(mut self, url: String) -> Self {
        self.config.return_url = Some(url);
        self
    }

    pub fn show_saved_payment_methods(mut self, show: bool) -> Self {
        self.config.show_saved_payment_methods = Some(show);
        self
    }

    pub fn tax_id(mut self, tax_id: String) -> Self {
        self.config.tax_id = Some(tax_id);
        self
    }

    pub fn trial_period_days(mut self, days: i32) -> Self {
        self.config.trial_period_days = Some(days);
        self
    }

    pub async fn send(self) -> Result<CreateSubscriptionResponse, crate::errors::Error> {
        Ok(CreateSubscription::orchestrate(self.handle, self.config).await?)
    }
}
