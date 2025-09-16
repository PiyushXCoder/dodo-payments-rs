use std::sync::Arc;
use reqwest::Method;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
    errors::Error,
};

use super::response::SubscriptionResponse;

pub struct GetSubscription;

impl GetSubscription {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        subscription_id: String,
    ) -> Result<SubscriptionResponse, Error> {
        let uri = format!("/subscriptions/{}", subscription_id);
        let response = make_reqwest(handle, Method::GET, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
