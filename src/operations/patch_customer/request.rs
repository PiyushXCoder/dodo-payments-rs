pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct PatchCustomer;

impl PatchCustomer {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        customer_id: String,
        config: PatchCustomerConfig,
    ) -> Result<PatchCustomerResponse, crate::errors::Error> {
        let path = format!("/customers/{}", customer_id);
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::PATCH, &path, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
