use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ListCustomersConfig {
    #[validate(minimum = 0)]
    #[validate(maximum = 100)]
    pub page_size: Option<i32>,
    #[validate(minimum = 0)]
    pub page_number: Option<i32>,
    pub email: Option<String>,
}
