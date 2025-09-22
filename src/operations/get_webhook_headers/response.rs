use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebhookHeadersResponse {
    pub headers: HashMap<String, String>,
    pub sensitive: Vec<String>,
}
