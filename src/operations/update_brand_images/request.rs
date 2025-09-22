pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct UpdateBrandImages;

impl UpdateBrandImages {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateBrandImagesConfig,
    ) -> Result<UpdateBrandImageResponse, crate::errors::Error> {
        let url = format!("/brands/{}/images", config.id);
        let response = make_reqwest(handle, Method::PUT, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}