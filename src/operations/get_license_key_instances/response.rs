use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseKeyInstanceResponse {
    pub business_id: String,
    pub created_at: String,
    pub id: String,
    pub license_key_id: String,
    pub name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListLicenseKeyInstancesResponse {
    pub items: Vec<LicenseKeyInstanceResponse>,
}
