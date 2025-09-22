use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListMetersConfig {
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    pub archived: Option<bool>,
}