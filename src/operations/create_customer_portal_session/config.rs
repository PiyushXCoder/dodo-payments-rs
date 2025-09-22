use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerPortalSessionConfig {
    pub send_email: Option<bool>,
}
