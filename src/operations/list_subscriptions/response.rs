use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::operations::common::structs::{BillingAddress, Currency};
use super::config::SubscriptionStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListSubscriptionsResponse {
    pub items: Vec<SubscriptionListResponseItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionListResponseItem {
    pub billing: BillingAddress,
    pub cancel_at_next_billing_date: bool,
    pub cancelled_at: Option<String>, // date-time
    pub created_at: String, // date-time
    pub currency: Currency,
    pub customer: CustomerLimitedDetailsResponse,
    pub discount_cycles_remaining: Option<i32>,
    pub discount_id: Option<String>,
    pub metadata: Metadata,
    pub next_billing_date: String, // date-time
    pub on_demand: bool,
    pub payment_frequency_count: i32,
    pub payment_frequency_interval: TimeInterval,
    pub previous_billing_date: String, // date-time
    pub product_id: String,
    pub quantity: i32,
    pub recurring_pre_tax_amount: i32,
    pub status: SubscriptionStatus,
    pub subscription_id: String,
    pub subscription_period_count: i32,
    pub subscription_period_interval: TimeInterval,
    pub tax_inclusive: bool,
    pub trial_period_days: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerLimitedDetailsResponse {
    pub customer_id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")] // OpenAPI spec uses PascalCase for enum values
pub enum TimeInterval {
    Day,
    Week,
    Month,
    Year,
}
