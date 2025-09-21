use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};

pub struct GetLicenseKey;

impl GetLicenseKey {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetLicenseKeyConfig,
    ) -> Result<GetLicenseKeyResponse, crate::errors::Error> {
        let url = format!("/license_keys/{}", config.id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
