use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct GetLicenseKeyInstance;

impl GetLicenseKeyInstance {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetLicenseKeyInstanceConfig,
    ) -> Result<GetLicenseKeyInstanceResponse, crate::errors::Error> {
        let url = format!("/license_key_instances/{}", config.id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
