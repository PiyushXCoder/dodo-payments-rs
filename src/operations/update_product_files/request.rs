pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct UpdateProductFiles;

impl UpdateProductFiles {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateProductFilesConfig,
    ) -> Result<UploadProductFileResponse, crate::errors::Error> {
        let url = format!("/products/{}/files", config.id);
        let body = serde_json::to_string(&config.body).unwrap();
        let response = make_reqwest(handle, Method::PUT, &url, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
