pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListEvents;

impl ListEvents {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListEventsConfig,
    ) -> Result<ListEventsResponse, crate::errors::Error> {
        // Build query parameters
        let mut query_params = Vec::new();
        
        if let Some(customer_id) = &config.customer_id {
            query_params.push(format!("customer_id={}", customer_id));
        }
        
        if let Some(event_name) = &config.event_name {
            query_params.push(format!("event_name={}", event_name));
        }
        
        if let Some(meter_id) = &config.meter_id {
            query_params.push(format!("meter_id={}", meter_id));
        }
        
        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }
        
        if let Some(start) = &config.start {
            query_params.push(format!("start={}", start));
        }
        
        if let Some(end) = &config.end {
            query_params.push(format!("end={}", end));
        }
        
        let query_string = if !query_params.is_empty() {
            format!("?{}", query_params.join("&"))
        } else {
            String::new()
        };
        
        let url = format!("/events{}", query_string);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}