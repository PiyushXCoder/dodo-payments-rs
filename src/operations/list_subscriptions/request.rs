use super::config::*;
use super::response::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListSubscriptions;

impl ListSubscriptions {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListSubscriptionsConfig,
    ) -> Result<ListSubscriptionsResponse, crate::errors::Error> {
        let mut url = "/subscriptions".to_string();
        let mut query_params = Vec::new();

        if let Some(created_at_gte) = config.created_at_gte {
            query_params.push(format!("created_at_gte={}", created_at_gte));
        }
        if let Some(created_at_lte) = config.created_at_lte {
            query_params.push(format!("created_at_lte={}", created_at_lte));
        }
        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }
        if let Some(customer_id) = config.customer_id {
            query_params.push(format!("customer_id={}", customer_id));
        }
        if let Some(status) = config.status {
            query_params.push(format!("status={}", serde_json::to_string(&status).unwrap().trim_matches('"')));
        }
        if let Some(brand_id) = config.brand_id {
            query_params.push(format!("brand_id={}", brand_id));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
