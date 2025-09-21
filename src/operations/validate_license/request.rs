use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct ValidateLicense;

impl ValidateLicense {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: ValidateLicenseConfig,
    ) -> Result<ValidateLicenseResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::POST, "/licenses/validate", Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
