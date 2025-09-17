use serde::{Deserialize, Serialize};
use super::super::common::structs::DiscountType;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDiscountConfig {
    #[serde(skip)]
    pub discount_id: String,
    pub amount: Option<i32>,
    pub code: Option<String>,
    pub expires_at: Option<String>,
    pub name: Option<String>,
    pub restricted_to: Option<Vec<String>>,
    pub subscription_cycles: Option<i32>,
    #[serde(rename = "type")]
    pub discount_type: Option<DiscountType>,
    pub usage_limit: Option<i32>,
}
