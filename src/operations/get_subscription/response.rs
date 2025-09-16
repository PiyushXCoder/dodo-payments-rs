use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::super::common::structs::{
    AddonCartResponseItem, BillingAddress, Currency,
    CustomerLimitedDetailsResponse, MeterCartResponseItem, SubscriptionStatus, TimeInterval,
};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionResponse {
    pub addons: Vec<AddonCartResponseItem>,
    pub billing: BillingAddress,
    pub cancel_at_next_billing_date: bool,
    pub cancelled_at: Option<String>, // Using String for date-time for simplicity, can be chrono::DateTime<Utc>
    pub created_at: String, // Using String for date-time
    pub currency: Currency,
    pub customer: CustomerLimitedDetailsResponse,
    pub discount_cycles_remaining: Option<i32>,
    pub discount_id: Option<String>,
    pub expires_at: Option<String>, // Using String for date-time
    pub metadata: HashMap<String, String>,
    pub meters: Vec<MeterCartResponseItem>,
    pub next_billing_date: String, // Using String for date-time
    pub on_demand: bool,
    pub payment_frequency_count: i32,
    pub payment_frequency_interval: TimeInterval,
    pub previous_billing_date: String, // Using String for date-time
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
