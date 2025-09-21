use serde::{Deserialize, Serialize};
use super::config::LicenseKeyStatus;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseKeyResponse {
    pub activations_limit: Option<i32>,
    pub business_id: String,
    pub created_at: String,
    pub customer_id: String,
    pub expires_at: Option<String>,
    pub id: String,
    pub instances_count: i32,
    pub key: String,
    pub payment_id: String,
    pub product_id: String,
    pub status: LicenseKeyStatus,
    pub subscription_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListLicenseKeysResponse {
    pub items: Vec<LicenseKeyResponse>,
}
