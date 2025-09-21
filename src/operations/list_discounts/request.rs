pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListDiscounts;

impl ListDiscounts {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListDiscountsConfig,
    ) -> Result<ListDiscountsResponse, crate::errors::Error> {
        let mut url = reqwest::Url::parse(&handle.config.environment.base_url())
            .unwrap()
            .join("/discounts")
            .unwrap();
        let mut query_params = Vec::new();

        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }

        if !query_params.is_empty() {
            url.set_query(Some(&query_params.join("&")));
        }

        let response = make_reqwest(handle, Method::GET, url.path(), None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
