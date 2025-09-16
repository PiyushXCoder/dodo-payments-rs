use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetLineItemsConfig {
    #[validate(min_length = 1)]
    pub payment_id: String,
}
