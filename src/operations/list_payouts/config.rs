use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListPayoutsConfig {
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
}