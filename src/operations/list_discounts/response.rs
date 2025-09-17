use serde::{Deserialize, Serialize};
use super::super::common::structs::DiscountResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDiscountsResponse {
    pub items: Vec<DiscountResponse>,
}
