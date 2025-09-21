use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ActivateLicenseConfig {
    pub license_key: String,
    pub name: String,
}
