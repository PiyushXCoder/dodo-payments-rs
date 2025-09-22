use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetWebhookHeadersConfig {
    #[validate(min_length = 1)]
    pub webhook_id: String,
}
