use super::super::common::structs::PayoutsResponse;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListPayoutsResponse {
    pub items: Vec<PayoutsResponse>,
}