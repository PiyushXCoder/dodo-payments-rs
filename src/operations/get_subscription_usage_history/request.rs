pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetSubscriptionUsageHistory;

impl GetSubscriptionUsageHistory {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetSubscriptionUsageHistoryConfig,
    ) -> Result<GetSubscriptionUsageHistoryResponse, crate::errors::Error> {
        let subscription_id = config.subscription_id.clone();
        let mut query_params = Vec::new();

        if let Some(start_date) = config.start_date {
            query_params.push(format!("start_date={}", start_date));
        }
        if let Some(end_date) = config.end_date {
            query_params.push(format!("end_date={}", end_date));
        }
        if let Some(meter_id) = config.meter_id {
            query_params.push(format!("meter_id={}", meter_id));
        }
        if let Some(page_size) = config.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_number) = config.page_number {
            query_params.push(format!("page_number={}", page_number));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let response = make_reqwest(
            handle,
            Method::GET,
            &format!("/subscriptions/{}/usage-history{}", subscription_id, query_string),
            None,
        )
        .await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
