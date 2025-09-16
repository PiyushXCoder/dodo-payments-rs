use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ListSubscriptionsConfig {
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    #[validate(minimum = 0)]
    pub page_size: Option<u32>,
    #[validate(minimum = 0)]
    pub page_number: Option<u32>,
    pub customer_id: Option<String>,
    pub status: Option<SubscriptionStatus>,
    pub brand_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    OnHold,
    Cancelled,
    Failed,
    Expired,
}
