use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Succeeded,
    Failed,
    Cancelled,
    Processing,
    RequiresCustomerAction,
    RequiresMerchantAction,
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresCapture,
    PartiallyCaptured,
    PartiallyCapturedAndCapturable,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPaymentsConfig {
    pub created_at_gte: Option<DateTime<Utc>>,
    pub created_at_lte: Option<DateTime<Utc>>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    pub customer_id: Option<String>,
    pub subscription_id: Option<String>,
    pub status: Option<Status>,
    pub brand_id: Option<String>,
}
