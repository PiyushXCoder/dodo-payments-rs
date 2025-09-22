#[allow(unused_imports)]
pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct UnarchiveMeter;

impl UnarchiveMeter {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        meter_id: String,
    ) -> Result<UnarchiveMeterResponse, crate::errors::Error> {
        let url = format!("/meters/{}/unarchive", meter_id);
        let response = make_reqwest(handle, Method::POST, &url, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}