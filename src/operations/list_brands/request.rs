#[allow(unused_imports)]
pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct ListBrands;

impl ListBrands {
    pub async fn orchestrate(
        handle: Arc<Handle>,
    ) -> Result<ListBrandsResponse, crate::errors::Error> {
        let response = make_reqwest(handle, Method::GET, "/brands", None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}