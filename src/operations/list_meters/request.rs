pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListMeters;

impl ListMeters {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListMetersConfig,
    ) -> Result<ListMetersResponse, crate::errors::Error> {
        // Build query parameters
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
        
        let query_string = if !query_params.is_empty() {
            format!("?{}", query_params.join("&"))
        } else {
            String::new()
        };
        
        let url = format!("/meters{}", query_string);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}