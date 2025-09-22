pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct UpdateProductImages;

impl UpdateProductImages {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateProductImagesConfig,
    ) -> Result<UpdateProductImageResponse, crate::errors::Error> {
        let mut url = format!("/products/{}/images", config.id);
        if let Some(force_update) = config.force_update {
            url = format!("{}?force_update={}", url, force_update);
        }
        let response = make_reqwest(handle, Method::PUT, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
