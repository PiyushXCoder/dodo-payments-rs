use super::super::common::structs::RefundListItem;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListRefundsResponse {
    pub items: Vec<RefundListItem>,
}