use super::super::common::structs::*;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CheckoutSessionsConfig {
    #[validate(min_items = 1)]
    pub product_cart: Vec<ProductItem>,

    pub customer: Option<CustomerInfo>,
    pub billing_address: Option<BillingAddress>,

    pub allowed_payment_method_types: Option<Vec<String>>,
    pub billing_currency: Option<String>,
    pub show_saved_payment_methods: Option<bool>,

    pub return_url: Option<String>,
    pub confirm: Option<bool>,
    pub discount_code: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub customization: Option<CheckoutSessionCustomization>,
    pub feature_flags: Option<CheckoutSessionFlags>,
    pub subscription_data: Option<SubscriptionData>,
}
