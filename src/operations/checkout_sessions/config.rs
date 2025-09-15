use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductItem {
    pub product_id: String,
    pub quantity: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub email: String,
    pub name: String,
    pub phone_number: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip_code: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionsConfig {
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
    // TODO: Add more fields as needed
}
