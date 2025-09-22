use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebhookSecretResponse {
    pub secret: String,
}
