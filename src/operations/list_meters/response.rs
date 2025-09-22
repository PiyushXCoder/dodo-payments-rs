use super::super::common::structs::MeterResponse;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListMetersResponse {
    pub items: Vec<MeterResponse>,
}