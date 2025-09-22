pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CreateCustomerWalletLedgerEntry;

impl CreateCustomerWalletLedgerEntry {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        customer_id: String,
        config: CreateCustomerWalletLedgerEntryConfig,
    ) -> Result<CreateCustomerWalletLedgerEntryResponse, crate::errors::Error> {
        let path = format!("/customers/{}/wallets/ledger-entries", customer_id);
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, &path, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
