use super::super::common::structs::PartialRefundItem;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRefundConfig {
    pub payment_id: String,
    pub items: Option<Vec<PartialRefundItem>>,
    pub reason: Option<String>,
}