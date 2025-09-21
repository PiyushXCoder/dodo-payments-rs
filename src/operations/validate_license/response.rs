use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateLicenseResponse {
    pub valid: bool,
}
