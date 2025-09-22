use super::super::common::structs::*;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomersResponse {
    pub items: Vec<CustomerInfoFull>,
}
