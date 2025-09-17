pub use super::config::*;
use super::*;
use reqwest::{Method, Url};
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UpdateDiscount;

impl UpdateDiscount {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateDiscountConfig,
    ) -> Result<UpdateDiscountResponse, crate::errors::Error> {
        let url = handle.config.environment.base_url();
        let url = Url::parse(&url)
            .unwrap()
            .join(&format!("/discounts/{}", config.discount_id))
            .unwrap();

        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::PATCH, url.path(), Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
