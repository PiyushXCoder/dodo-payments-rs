use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChargeResponse {
    pub payment_id: String,
}
