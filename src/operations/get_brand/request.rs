use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct GetBrand;

impl GetBrand {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        brand_id: String,
    ) -> Result<GetBrandResponse, crate::errors::Error> {
        let url = format!("/brands/{}", brand_id);
        let response = make_reqwest(handle, Method::GET, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}