pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UpdateAddon;

impl UpdateAddon {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        addon_id: String,
        config: UpdateAddonConfig,
    ) -> Result<UpdateAddonResponse, crate::errors::Error> {
        let body = serde_json::to_string(&config).unwrap();
        let url = format!("/addons/{}", addon_id);
        let response = make_reqwest(handle, Method::PATCH, &url, Some(body)).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}