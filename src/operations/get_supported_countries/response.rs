use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSupportedCountriesResponse(pub Vec<crate::operations::common::structs::CountryCodeAlpha2>);
