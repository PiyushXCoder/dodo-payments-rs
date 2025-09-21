use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct UpdateLicenseKeyInstance;

impl UpdateLicenseKeyInstance {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: UpdateLicenseKeyInstanceConfig,
    ) -> Result<UpdateLicenseKeyInstanceResponse, crate::errors::Error> {
        let id = config.id.clone();
        let url = format!("/license_key_instances/{}", id);
        let body = serde_json::to_string(&config).unwrap();
        let response = make_reqwest(handle, Method::PATCH, &url, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
