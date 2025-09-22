use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use crate::operations::common::structs::Currency;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ListCustomerWalletLedgerEntriesConfig {
    #[validate(minimum = 0)]
    pub page_size: Option<i32>,
    #[validate(minimum = 0)]
    pub page_number: Option<i32>,
    pub currency: Option<Currency>,
}
