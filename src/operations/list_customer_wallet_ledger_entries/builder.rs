use super::*;
use crate::{client::Handle, operations::list_customer_wallet_ledger_entries::ListCustomerWalletLedgerEntriesResponse};

use std::sync::Arc;

pub struct ListCustomerWalletLedgerEntriesBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
    pub config: ListCustomerWalletLedgerEntriesConfig,
}

impl ListCustomerWalletLedgerEntriesBuilder {
    pub fn new(handle: Arc<Handle>, customer_id: String) -> Self {
        Self {
            handle,
            customer_id,
            config: ListCustomerWalletLedgerEntriesConfig {
                page_size: None,
                page_number: None,
                currency: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: i32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub fn currency(mut self, currency: crate::operations::common::structs::Currency) -> Self {
        self.config.currency = Some(currency);
        self
    }

    pub async fn send(self) -> Result<ListCustomerWalletLedgerEntriesResponse, crate::errors::Error> {
        Ok(ListCustomerWalletLedgerEntries::orchestrate(self.handle, self.customer_id, self.config).await?)
    }
}
