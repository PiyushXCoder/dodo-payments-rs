use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetLicenseKeysConfig {
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
    pub customer_id: Option<String>,
    pub status: Option<LicenseKeyStatus>,
    pub product_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LicenseKeyStatus {
    Active,
    Expired,
    Disabled,
}
