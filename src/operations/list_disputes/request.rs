pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListDisputes;

impl ListDisputes {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListDisputesConfig,
    ) -> Result<ListDisputesResponse, crate::errors::Error> {
        // Build query parameters
        let mut query_params = Vec::new();
        
        if let Some(created_at_gte) = &config.created_at_gte {
            query_params.push(format!("created_at_gte={}", created_at_gte));
        }
        
        if let Some(created_at_lte) = &config.created_at_lte {
            query_params.push(format!("created_at_lte={}", created_at_lte));
        }
        
        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }
        
        if let Some(dispute_status) = &config.dispute_status {
            query_params.push(format!("dispute_status={}", dispute_status));
        }
        
        if let Some(dispute_stage) = &config.dispute_stage {
            query_params.push(format!("dispute_stage={}", dispute_stage));
        }
        
        if let Some(customer_id) = &config.customer_id {
            query_params.push(format!("customer_id={}", customer_id));
        }
        
        let query_string = if !query_params.is_empty() {
            format!("?{}", query_params.join("&"))
        } else {
            String::new()
        };
        
        let url = format!("/disputes{}", query_string);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}