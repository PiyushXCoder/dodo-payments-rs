pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CreateMeter;

impl CreateMeter {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: CreateMeterConfig,
    ) -> Result<CreateMeterResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, "/meters", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}