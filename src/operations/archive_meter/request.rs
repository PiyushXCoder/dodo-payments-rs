#[allow(unused_imports)]
pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ArchiveMeter;

impl ArchiveMeter {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        meter_id: String,
    ) -> Result<ArchiveMeterResponse, crate::errors::Error> {
        let url = format!("/meters/{}", meter_id);
        let response = make_reqwest(handle, Method::DELETE, &url, None).await?;
        
        // Handle 204 No Content response
        if response.status().as_u16() == 204 {
            Ok(ArchiveMeterResponse {})
        } else {
            let text = response.text().await?;
            parse_response(&text)
        }
    }
}