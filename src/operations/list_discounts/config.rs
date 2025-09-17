use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ListDiscountsConfig {
    #[validate(minimum = 0)]
    pub page_size: Option<u32>,
    #[validate(minimum = 0)]
    pub page_number: Option<u32>,
}
