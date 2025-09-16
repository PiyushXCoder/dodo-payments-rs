use super::super::common::structs::Currency;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubscriptionUsageHistoryResponse {
    pub items: Vec<UsageHistoryItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageHistoryItem {
    pub end_date: String, // TODO: Use chrono for date-time
    pub meters: Vec<MeterUsageItem>,
    pub start_date: String, // TODO: Use chrono for date-time
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MeterUsageItem {
    pub chargeable_units: String,
    pub consumed_units: String,
    pub currency: Currency,
    pub free_threshold: i64,
    pub id: String,
    pub name: String,
    pub price_per_unit: String,
    pub total_price: i32,
}
