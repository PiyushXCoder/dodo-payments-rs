use std::sync::Arc;
use crate::{client::Handle, operations::get_supported_countries::GetSupportedCountries};

pub struct GetSupportedCountriesBuilder {
    pub(crate) handle: Arc<Handle>,
}

impl GetSupportedCountriesBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self { handle }
    }

    pub async fn send(self) -> Result<Vec<crate::operations::common::structs::CountryCodeAlpha2>, crate::errors::Error> {
        Ok(GetSupportedCountries::orchestrate(self.handle).await?)
    }
}
