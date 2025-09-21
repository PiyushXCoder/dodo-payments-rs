use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct GetLicenseKeys;

impl GetLicenseKeys {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetLicenseKeysConfig,
    ) -> Result<ListLicenseKeysResponse, crate::errors::Error> {
        let mut url = "/license_keys".to_string();
        let mut query_params = Vec::new();

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
        if let Some(product_id) = config.product_id {
            query_params.push(format!("product_id={}", product_id));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
