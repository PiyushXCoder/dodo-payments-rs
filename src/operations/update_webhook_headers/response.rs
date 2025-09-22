use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWebhookHeadersResponse {
    pub message: String,
}
