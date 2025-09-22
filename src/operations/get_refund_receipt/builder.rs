use super::*;
use crate::{client::Handle, operations::get_refund_receipt::GetRefundReceiptResponse};

use std::sync::Arc;

pub struct GetRefundReceiptBuilder {
    pub refund_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetRefundReceiptBuilder {
    pub fn new(handle: Arc<Handle>, refund_id: String) -> Self {
        Self {
            handle,
            refund_id,
        }
    }

    pub async fn send(self) -> Result<GetRefundReceiptResponse, crate::errors::Error> {
        Ok(GetRefundReceipt::orchestrate(self.handle, self.refund_id).await?)
    }
}