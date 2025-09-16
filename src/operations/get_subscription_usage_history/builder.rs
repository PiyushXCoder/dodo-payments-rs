use super::{GetSubscriptionUsageHistoryConfig, GetSubscriptionUsageHistoryResponse};
use crate::{client::Handle, operations::get_subscription_usage_history::GetSubscriptionUsageHistory};

use std::sync::Arc;

pub struct GetSubscriptionUsageHistoryBuilder {
    pub config: GetSubscriptionUsageHistoryConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetSubscriptionUsageHistoryBuilder {
    pub fn new(handle: Arc<Handle>, subscription_id: String) -> Self {
        Self {
            handle,
            config: GetSubscriptionUsageHistoryConfig {
                subscription_id,
                start_date: None,
                end_date: None,
                meter_id: None,
                page_size: None,
                page_number: None,
            },
        }
    }

    pub fn start_date(mut self, date: String) -> Self {
        self.config.start_date = Some(date);
        self
    }

    pub fn end_date(mut self, date: String) -> Self {
        self.config.end_date = Some(date);
        self
    }

    pub fn meter_id(mut self, meter_id: String) -> Self {
        self.config.meter_id = Some(meter_id);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: i32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub async fn send(self) -> Result<GetSubscriptionUsageHistoryResponse, crate::errors::Error> {
        Ok(GetSubscriptionUsageHistory::orchestrate(self.handle, self.config).await?)
    }
}
