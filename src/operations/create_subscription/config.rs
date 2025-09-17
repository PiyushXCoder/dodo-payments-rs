use super::super::common::structs::*;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::operations::common::structs::BillingAddress;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachAddonReq {
    pub addon_id: String,
    pub quantity: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnDemandSubscriptionReq {
    pub mandate_only: bool,
    pub adaptive_currency_fees_inclusive: Option<bool>,
    pub product_currency: Option<Currency>,
    pub product_description: Option<String>,
    pub product_price: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestMetadata(pub HashMap<String, String>);

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateSubscriptionConfig {
    pub product_id: String,
    #[validate(minimum = 0)]
    pub quantity: i32,
    pub customer: CustomerRequest,
    pub billing: BillingAddress,
    pub addons: Option<Vec<AttachAddonReq>>,
    pub allowed_payment_method_types: Option<Vec<PaymentMethodTypes>>,
    pub billing_currency: Option<Currency>,
    pub discount_code: Option<String>,
    pub metadata: Option<RequestMetadata>,
    pub on_demand: Option<OnDemandSubscriptionReq>,
    pub payment_link: Option<bool>,
    pub return_url: Option<String>,
    pub show_saved_payment_methods: Option<bool>,
    pub tax_id: Option<String>,
    #[validate(minimum = 0)]
    pub trial_period_days: Option<i32>,
}
