use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use super::super::common::structs::DiscountType;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateDiscountConfig {
    #[validate(minimum = 1)]
    pub amount: i32,
    pub code: Option<String>,
    pub expires_at: Option<String>,
    pub name: Option<String>,
    pub restricted_to: Option<Vec<String>>,
    pub subscription_cycles: Option<i32>,
    #[serde(rename = "type")]
    pub discount_type: DiscountType,
    #[validate(minimum = 1)]
    pub usage_limit: Option<i32>,
}
