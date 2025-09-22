use super::super::common::{make_reqwest, parse_response};
use reqwest::Method;
use std::sync::Arc;

use crate::client::Handle;

pub struct GetSupportedCountries;

impl GetSupportedCountries {
    pub async fn orchestrate(
        handle: Arc<Handle>,
    ) -> Result<Vec<crate::operations::common::structs::CountryCodeAlpha2>, crate::errors::Error> {
        let response = make_reqwest(handle, Method::GET, "/checkout/supported_countries", None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
