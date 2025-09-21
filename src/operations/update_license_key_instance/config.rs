use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLicenseKeyInstanceConfig {
    #[serde(skip)]
    pub id: String,
    pub name: String,
}
