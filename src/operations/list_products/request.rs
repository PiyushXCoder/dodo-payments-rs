pub use super::config::*;
use super::response::GetProductsListResponse;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct ListProducts;

impl ListProducts {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListProductsConfig,
    ) -> Result<GetProductsListResponse, crate::errors::Error> {
        let mut query_params = Vec::new();

        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }
        if let Some(archived) = config.archived {
            query_params.push(format!("archived={}", archived));
        }
        if let Some(recurring) = config.recurring {
            query_params.push(format!("recurring={}", recurring));
        }
        if let Some(brand_id) = config.brand_id {
            query_params.push(format!("brand_id={}", brand_id));
        }

        let query = if query_params.is_empty() {
            None
        } else {
            Some(query_params.join("&"))
        };

        let url = if let Some(q) = query {
            format!("/products?{}", q)
        } else {
            "/products".to_string()
        };

        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
