use super::*;
use crate::{client::Handle, operations::get_customer::GetCustomerResponse};

use std::sync::Arc;

pub struct GetCustomerBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
}

impl GetCustomerBuilder {
    pub fn new(handle: Arc<Handle>, customer_id: String) -> Self {
        Self {
            handle,
            customer_id,
        }
    }

    pub async fn send(self) -> Result<GetCustomerResponse, crate::errors::Error> {
        Ok(GetCustomer::orchestrate(self.handle, self.customer_id).await?)
    }
}
