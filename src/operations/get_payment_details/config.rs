use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPaymentDetailsConfig {
    pub payment_id: String,
}
