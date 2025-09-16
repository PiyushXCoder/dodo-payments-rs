pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetLineItems;

impl GetLineItems {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetLineItemsConfig,
    ) -> Result<GetLineItemsResponse, crate::errors::Error> {
        let uri = format!("/payments/{}/line-items", config.payment_id);
        let response = make_reqwest(handle, Method::GET, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
