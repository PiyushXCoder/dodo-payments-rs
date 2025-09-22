use super::*;
use crate::{client::Handle, operations::get_refund::GetRefundResponse};

use std::sync::Arc;

pub struct GetRefundBuilder {
    pub refund_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetRefundBuilder {
    pub fn new(handle: Arc<Handle>, refund_id: String) -> Self {
        Self {
            handle,
            refund_id,
        }
    }

    pub async fn send(self) -> Result<GetRefundResponse, crate::errors::Error> {
        Ok(GetRefund::orchestrate(self.handle, self.refund_id).await?)
    }
}