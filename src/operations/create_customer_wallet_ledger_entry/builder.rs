use super::*;
use crate::{client::Handle, operations::create_customer_wallet_ledger_entry::CreateCustomerWalletLedgerEntryResponse};
use crate::operations::common::structs::Currency;

use std::sync::Arc;

pub struct CreateCustomerWalletLedgerEntryBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
    pub config: CreateCustomerWalletLedgerEntryConfig,
}

impl CreateCustomerWalletLedgerEntryBuilder {
    pub fn new(
        handle: Arc<Handle>,
        customer_id: String,
        amount: i64,
        currency: Currency,
        entry_type: CustomerLedgerEntryType,
    ) -> Self {
        Self {
            handle,
            customer_id,
            config: CreateCustomerWalletLedgerEntryConfig {
                amount,
                currency,
                entry_type,
                idempotency_key: None,
                reason: None,
            },
        }
    }

    pub fn idempotency_key(mut self, idempotency_key: String) -> Self {
        self.config.idempotency_key = Some(idempotency_key);
        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.config.reason = Some(reason);
        self
    }

    pub async fn send(self) -> Result<CreateCustomerWalletLedgerEntryResponse, crate::errors::Error> {
        Ok(CreateCustomerWalletLedgerEntry::orchestrate(self.handle, self.customer_id, self.config).await?)
    }
}
