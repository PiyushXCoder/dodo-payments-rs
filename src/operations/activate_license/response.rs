use serde::{Deserialize, Serialize};
use super::super::common::structs::CustomerLimitedDetailsResponse;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivateLicenseKeyProductInfo {
    pub name: Option<String>,
    pub product_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivateLicenseResponse {
    pub business_id: String,
    pub created_at: String,
    pub customer: CustomerLimitedDetailsResponse,
    pub id: String,
    pub license_key_id: String,
    pub name: String,
    pub product: ActivateLicenseKeyProductInfo,
}
