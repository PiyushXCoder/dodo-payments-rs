pub use super::config::*;
pub use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct ListWebhooks;

impl ListWebhooks {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListWebhooksConfig,
    ) -> Result<ListWebhooksResponse, crate::errors::Error> {
        let mut url = "/webhooks".to_string();
        
        let mut query_params = Vec::new();
        if let Some(limit) = config.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(iterator) = &config.iterator {
            query_params.push(format!("iterator={}", iterator));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}