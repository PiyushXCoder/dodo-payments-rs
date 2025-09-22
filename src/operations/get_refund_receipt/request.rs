#[allow(unused_imports)]
pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::make_reqwest,
};

pub struct GetRefundReceipt;

impl GetRefundReceipt {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        refund_id: String,
    ) -> Result<GetRefundReceiptResponse, crate::errors::Error> {
        let url = format!("/invoices/refunds/{}", refund_id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}