use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSessionsResponse {
    pub session_id: String,
    pub checkout_url: String,
}
