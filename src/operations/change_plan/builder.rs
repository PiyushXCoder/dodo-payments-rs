use super::*;
use crate::{client::Handle, operations::change_plan::ChangePlanResponse};
use std::sync::Arc;

pub struct ChangePlanBuilder {
    pub config: ChangePlanConfig,
    pub(crate) handle: Arc<Handle>,
    pub(crate) subscription_id: String,
}

impl ChangePlanBuilder {
    pub fn new(handle: Arc<Handle>, subscription_id: String, product_id: String, proration_billing_mode: ProrationBillingMode, quantity: i32) -> Self {
        Self {
            handle,
            subscription_id,
            config: ChangePlanConfig {
                product_id,
                proration_billing_mode,
                quantity,
                addons: None,
            },
        }
    }

    pub fn addons(mut self, addons: Vec<AttachAddonReq>) -> Self {
        self.config.addons = Some(addons);
        self
    }

    pub async fn send(self) -> Result<ChangePlanResponse, crate::errors::Error> {
        Ok(ChangePlan::orchestrate(self.handle, self.subscription_id, self.config).await?)
    }
}
