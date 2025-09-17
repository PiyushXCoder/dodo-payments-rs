pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CreateDiscount;

impl CreateDiscount {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CreateDiscountConfig,
    ) -> Result<CreateDiscountResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, "/discounts", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
