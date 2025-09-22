pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetAddon;

impl GetAddon {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        addon_id: String,
    ) -> Result<GetAddonResponse, crate::errors::Error> {
        let url = format!("/addons/{}", addon_id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}