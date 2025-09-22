pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct ArchiveProduct;

impl ArchiveProduct {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ArchiveProductConfig,
    ) -> Result<ArchiveProductResponse, crate::errors::Error> {
        let url = format!("/products/{}", config.id);
        let response = make_reqwest(handle, Method::DELETE, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
