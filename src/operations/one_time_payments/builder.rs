use super::super::common::structs::*;
use super::*;
use crate::{client::Handle, operations::one_time_payments::CreateOneTimePaymentResponse};
use std::{collections::HashMap, sync::Arc};

pub struct OneTimePaymentBuilder {
    pub config: CreateOneTimePaymentRequest,
    pub(crate) handle: Arc<Handle>,
}

impl OneTimePaymentBuilder {
    pub fn new(
        handle: Arc<Handle>,
        product_cart: Vec<ProductItem>,
        customer: CustomerRequest,
        billing: BillingAddress,
    ) -> Self {
        Self {
            handle,
            config: CreateOneTimePaymentRequest {
                product_cart,
                customer,
                billing,
                allowed_payment_method_types: None,
                billing_currency: None,
                discount_code: None,
                metadata: None,
                payment_link: None,
                return_url: None,
                show_saved_payment_methods: None,
                tax_id: None,
            },
        }
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
        self.config.metadata = Some(metadata);
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

    pub async fn send(self) -> Result<CreateOneTimePaymentResponse, crate::errors::Error> {
        Ok(OneTimePayments::orchestrate(self.handle, self.config).await?)
    }
}
