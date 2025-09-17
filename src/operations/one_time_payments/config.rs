use super::super::common::structs::*;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateOneTimePaymentRequest {
    #[validate(min_items = 1)]
    pub product_cart: Vec<ProductItem>,
    pub customer: CustomerRequest,
    pub billing: BillingAddress,
    pub allowed_payment_method_types: Option<Vec<PaymentMethodTypes>>,
    pub billing_currency: Option<Currency>,
    pub discount_code: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub payment_link: Option<bool>,
    pub return_url: Option<String>,
    pub show_saved_payment_methods: Option<bool>,
    pub tax_id: Option<String>,
}
