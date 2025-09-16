pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UpdateSubscription;

impl UpdateSubscription {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateSubscriptionConfig,
    ) -> Result<UpdateSubscriptionResponse, crate::errors::Error> {
        let subscription_id = config.subscription_id.clone();
        println!("Updating subscription with ID: {}", subscription_id);
        let body = serde_json::to_string(&config).unwrap();
        println!("Request Body: {}", body);
        let response = make_reqwest(
            handle,
            Method::PATCH,
            &format!("/subscriptions/{}", subscription_id),
            Some(body),
        )
        .await?;
        let text = response.text().await?;
        println!("Response Text: {}", text);
        parse_response(&text)
    }
}
