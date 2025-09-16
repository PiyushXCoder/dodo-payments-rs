use super::super::common::structs::*;
use super::config::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaymentsResponse {
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub brand_id: String,
    pub created_at: DateTime<Utc>,
    pub currency: Currency,
    pub customer: CustomerInfo,
    pub digital_products_delivered: bool,
    pub metadata: Value,
    pub payment_id: String,
    pub total_amount: i64,
    pub payment_method: Option<String>,
    pub payment_method_type: Option<String>,
    pub status: Option<Status>,
    pub subscription_id: Option<String>,
}
