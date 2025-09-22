pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct UpdateBrand;

impl UpdateBrand {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateBrandConfig,
    ) -> Result<UpdateBrandResponse, crate::errors::Error> {
        let url = format!("/brands/{}", config.id);
        let body = serde_json::to_string(&config.body).unwrap();
        let response = make_reqwest(handle, Method::PATCH, &url, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}