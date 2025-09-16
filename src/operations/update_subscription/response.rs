use super::super::common::structs::{AddonCartResponseItem, BillingAddress, Currency, CustomerLimitedDetailsResponse, Metadata, MeterCartResponseItem, SubscriptionStatus, TimeInterval};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriptionResponse {
    pub addons: Option<Vec<AddonCartResponseItem>>,
    pub billing: Option<BillingAddress>,
    pub cancel_at_next_billing_date: Option<bool>,
    pub cancelled_at: Option<String>, // TODO: Use chrono for date-time
    pub created_at: String, // TODO: Use chrono for date-time
    pub currency: Currency,
    pub customer: CustomerLimitedDetailsResponse,
    pub discount_cycles_remaining: Option<i32>,
    pub discount_id: Option<String>,
    pub expires_at: Option<String>, // TODO: Use chrono for date-time
    pub metadata: Option<Metadata>,
    pub meters: Option<Vec<MeterCartResponseItem>>,
    pub next_billing_date: String, // TODO: Use chrono for date-time
    pub on_demand: bool,
    pub payment_frequency_count: i32,
    pub payment_frequency_interval: TimeInterval,
    pub previous_billing_date: String, // TODO: Use chrono for date-time
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
