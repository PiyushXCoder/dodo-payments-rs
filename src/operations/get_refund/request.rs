pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetRefund;

impl GetRefund {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        refund_id: String,
    ) -> Result<GetRefundResponse, crate::errors::Error> {
        let url = format!("/refunds/{}", refund_id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}