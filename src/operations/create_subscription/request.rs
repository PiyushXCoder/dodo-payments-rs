pub use super::config::*;
use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
    operations::create_subscription::response::CreateSubscriptionResponse,
};
use reqwest::Method;
use std::sync::Arc;

pub struct CreateSubscription;

impl CreateSubscription {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CreateSubscriptionConfig,
    ) -> Result<CreateSubscriptionResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        println!("Request Body: {}", body); // Debug print
        let response = make_reqwest(handle, Method::POST, "/subscriptions", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
