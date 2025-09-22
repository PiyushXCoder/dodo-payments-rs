use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UpdateAddonImages;

impl UpdateAddonImages {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        addon_id: String,
    ) -> Result<UpdateAddonImagesResponse, crate::errors::Error> {
        let url = format!("/addons/{}/images", addon_id);
        let response = make_reqwest(handle, Method::PUT, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}

