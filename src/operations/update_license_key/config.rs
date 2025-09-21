use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLicenseKeyConfig {
    #[serde(skip)]
    pub id: String,
    pub activations_limit: Option<Option<i32>>,
    pub disabled: Option<Option<bool>>,
    pub expires_at: Option<Option<String>>,
}
