use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetSubscriptionUsageHistoryConfig {
    pub subscription_id: String,
    pub start_date: Option<String>, // TODO: Use chrono for date-time
    pub end_date: Option<String>,   // TODO: Use chrono for date-time
    pub meter_id: Option<String>,
    #[validate(minimum = 0)]
    pub page_size: Option<i32>,
    #[validate(minimum = 0)]
    pub page_number: Option<i32>,
}
