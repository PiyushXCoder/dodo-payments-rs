use std::sync::Arc;

use crate::{client::Handle, errors::Error};

use super::{request::GetSubscription, response::SubscriptionResponse};

pub struct GetSubscriptionBuilder {
    pub(crate) handle: Arc<Handle>,
    pub(crate) subscription_id: String,
}

impl GetSubscriptionBuilder {
    pub fn new(handle: Arc<Handle>, subscription_id: String) -> Self {
        Self {
            handle,
            subscription_id,
        }
    }

    pub async fn send(self) -> Result<SubscriptionResponse, Error> {
        GetSubscription::orchestrate(self.handle, self.subscription_id).await
    }
}
