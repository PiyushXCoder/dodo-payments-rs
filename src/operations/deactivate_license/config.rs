use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeactivateLicenseConfig {
    pub license_key: String,
    pub license_key_instance_id: String,
}
