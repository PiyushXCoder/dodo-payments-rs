use super::*;
use crate::{client::Handle, operations::get_dispute::GetDisputeResponseItem};

use std::sync::Arc;

pub struct GetDisputeBuilder {
    pub dispute_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetDisputeBuilder {
    pub fn new(handle: Arc<Handle>, dispute_id: String) -> Self {
        Self {
            handle,
            dispute_id,
        }
    }

    pub async fn send(self) -> Result<GetDisputeResponseItem, crate::errors::Error> {
        Ok(GetDispute::orchestrate(self.handle, self.dispute_id).await?)
    }
}