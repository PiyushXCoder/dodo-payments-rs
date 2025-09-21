use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct GetLicenseKeyInstances;

impl GetLicenseKeyInstances {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetLicenseKeyInstancesConfig,
    ) -> Result<ListLicenseKeyInstancesResponse, crate::errors::Error> {
        let mut url = "/license_key_instances".to_string();
        let mut query_params = Vec::new();

        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }
        if let Some(license_key_id) = config.license_key_id {
            query_params.push(format!("license_key_id={}", license_key_id));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
