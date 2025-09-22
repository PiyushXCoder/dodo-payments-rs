use super::super::common::structs::ListDisputeResponse;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDisputesResponse {
    pub items: Vec<ListDisputeResponse>,
}