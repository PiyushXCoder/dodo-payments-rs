use serde::{Deserialize, Serialize};
use crate::operations::common::structs::Currency;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CustomerLedgerEntryType {
    Credit,
    Debit,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerWalletLedgerEntryConfig {
    pub amount: i64,
    pub currency: Currency,
    pub entry_type: CustomerLedgerEntryType,
    pub idempotency_key: Option<String>,
    pub reason: Option<String>,
}
