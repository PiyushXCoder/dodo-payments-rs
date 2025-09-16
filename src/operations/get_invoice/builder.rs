use super::*;
use crate::{client::Handle, operations::get_invoice::GetInvoiceResponse};

use std::sync::Arc;

pub struct GetInvoiceBuilder {
    pub(crate) handle: Arc<Handle>,
    pub(crate) payment_id: String,
}

impl GetInvoiceBuilder {
    pub fn new(handle: Arc<Handle>, payment_id: String) -> Self {
        Self {
            handle,
            payment_id,
        }
    }

    pub async fn send(self) -> Result<GetInvoiceResponse, crate::errors::Error> {
        Ok(GetInvoice::orchestrate(self.handle, self.payment_id).await?)
    }
}
