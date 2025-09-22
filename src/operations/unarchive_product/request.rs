pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct UnarchiveProduct;

impl UnarchiveProduct {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UnarchiveProductConfig,
    ) -> Result<UnarchiveProductResponse, crate::errors::Error> {
        let url = format!("/products/{}/unarchive", config.id);
        let response = make_reqwest(handle, Method::POST, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
