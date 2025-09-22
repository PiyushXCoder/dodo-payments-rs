use super::*;
use crate::{client::Handle, operations::get_customer_wallets::GetCustomerWalletsResponse};

use std::sync::Arc;

pub struct GetCustomerWalletsBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
}

impl GetCustomerWalletsBuilder {
    pub fn new(handle: Arc<Handle>, customer_id: String) -> Self {
        Self {
            handle,
            customer_id,
        }
    }

    pub async fn send(self) -> Result<GetCustomerWalletsResponse, crate::errors::Error> {
        Ok(GetCustomerWallets::orchestrate(self.handle, self.customer_id).await?)
    }
}
