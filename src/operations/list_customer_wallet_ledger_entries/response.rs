use serde::{Deserialize, Serialize};
use crate::operations::common::structs::Currency;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CustomerLedgerEventType {
    Payment,
    PaymentReversal,
    Refund,
    RefundReversal,
    Dispute,
    DisputeReversal,
    MerchantAdjustment,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerWalletTransactionResponse {
    pub after_balance: i64,
    pub amount: i64,
    pub before_balance: i64,
    pub business_id: String,
    pub created_at: String,
    pub currency: Currency,
    pub customer_id: String,
    pub event_type: CustomerLedgerEventType,
    pub id: String,
    pub is_credit: bool,
    pub reason: Option<String>,
    pub reference_object_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomerWalletLedgerEntriesResponse {
    pub items: Vec<CustomerWalletTransactionResponse>,
}
