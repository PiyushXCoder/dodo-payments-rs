use super::super::common::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLineItem {
    pub amount: i32,
    pub description: Option<String>,
    pub items_id: String,
    pub name: Option<String>,
    pub refundable_amount: i32,
    pub tax: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLineItemsResponse {
    pub currency: Currency,
    pub items: Vec<PaymentLineItem>,
}
