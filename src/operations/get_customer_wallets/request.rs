pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetCustomerWallets;

impl GetCustomerWallets {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        customer_id: String,
    ) -> Result<GetCustomerWalletsResponse, crate::errors::Error> {
        let path = format!("/customers/{}/wallets", customer_id);
        let response = make_reqwest(handle, Method::GET, &path, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
