pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct CreateProduct;

impl CreateProduct {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CreateProductRequest,
    ) -> Result<GetProductResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, "/products", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
