pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListPayments;

impl ListPayments {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ListPaymentsConfig,
    ) -> Result<ListPaymentsResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::GET, "/payments", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
