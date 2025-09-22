use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWebhookHeadersConfig {
    #[serde(skip)]
    #[validate(min_length = 1)]
    pub webhook_id: String,
    pub headers: HashMap<String, String>,
}
