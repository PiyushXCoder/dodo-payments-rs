use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetLicenseKeyInstancesConfig {
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
    pub license_key_id: Option<String>,
}
