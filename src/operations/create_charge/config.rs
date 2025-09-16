use super::super::common::structs::{Currency, Metadata};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateChargeConfig {
    pub subscription_id: String,
    pub product_price: i32,
    pub adaptive_currency_fees_inclusive: Option<bool>,
    pub customer_balance_config: Option<CustomerBalanceConfig>,
    pub metadata: Option<Metadata>,
    pub product_currency: Option<Currency>,
    pub product_description: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CustomerBalanceConfig {
    pub allow_customer_credits_purchase: Option<bool>,
    pub allow_customer_credits_usage: Option<bool>,
}
