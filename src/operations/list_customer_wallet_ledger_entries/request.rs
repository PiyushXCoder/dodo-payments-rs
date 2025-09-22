pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListCustomerWalletLedgerEntries;

impl ListCustomerWalletLedgerEntries {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        customer_id: String,
        config: ListCustomerWalletLedgerEntriesConfig,
    ) -> Result<ListCustomerWalletLedgerEntriesResponse, crate::errors::Error> {
        let query = serde_qs::to_string(&config).unwrap();
        let path = format!("/customers/{}/wallets/ledger-entries?{}", customer_id, query);
        let response = make_reqwest(handle, Method::GET, &path, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
