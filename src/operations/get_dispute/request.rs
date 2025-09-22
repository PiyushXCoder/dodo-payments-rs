#[allow(unused_imports)]
pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetDispute;

impl GetDispute {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        dispute_id: String,
    ) -> Result<GetDisputeResponseItem, crate::errors::Error> {
        let url = format!("/disputes/{}", dispute_id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}