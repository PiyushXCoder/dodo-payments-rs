use super::super::common::structs::{BillingAddress, Metadata, SubscriptionStatus};
use super::{UpdateSubscriptionConfig, UpdateSubscriptionResponse};
use crate::{client::Handle, operations::update_subscription::UpdateSubscription};

use std::{sync::Arc};

pub struct UpdateSubscriptionBuilder {
    pub config: UpdateSubscriptionConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateSubscriptionBuilder {
    pub fn new(handle: Arc<Handle>, subscription_id: String) -> Self {
        Self {
            handle,
            config: UpdateSubscriptionConfig {
                subscription_id,
                billing: None,
                cancel_at_next_billing_date: None,
                disable_on_demand: None,
                metadata: None,
                next_billing_date: None,
                status: None,
                tax_id: None,
            },
        }
    }

    pub fn billing(mut self, billing: BillingAddress) -> Self {
        self.config.billing = Some(billing);
        self
    }

    pub fn cancel_at_next_billing_date(mut self, cancel: bool) -> Self {
        self.config.cancel_at_next_billing_date = Some(cancel);
        self
    }

    pub fn disable_on_demand(mut self, disable: super::DisableOnDemandReq) -> Self {
        self.config.disable_on_demand = Some(disable);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.config.metadata = Some(metadata);
        self
    }

    pub fn next_billing_date(mut self, date: String) -> Self {
        self.config.next_billing_date = Some(date);
        self
    }

    pub fn status(mut self, status: SubscriptionStatus) -> Self {
        self.config.status = Some(status);
        self
    }

    pub fn tax_id(mut self, tax_id: String) -> Self {
        self.config.tax_id = Some(tax_id);
        self
    }

    pub async fn send(self) -> Result<UpdateSubscriptionResponse, crate::errors::Error> {
        Ok(UpdateSubscription::orchestrate(self.handle, self.config).await?)
    }
}
