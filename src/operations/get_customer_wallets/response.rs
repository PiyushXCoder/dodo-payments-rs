use serde::{Deserialize, Serialize};
use crate::operations::common::structs::Currency;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerWalletResponse {
    pub balance: i64,
    pub created_at: String,
    pub currency: Currency,
    pub customer_id: String,
    pub updated_at: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomerWalletsResponse {
    pub items: Vec<CustomerWalletResponse>,
    pub total_balance_usd: i64,
}
