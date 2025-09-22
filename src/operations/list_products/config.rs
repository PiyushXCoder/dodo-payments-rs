use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListProductsConfig {
    pub page_size: Option<i64>,
    pub page_number: Option<i64>,
    pub archived: Option<bool>,
    pub recurring: Option<bool>,
    pub brand_id: Option<String>,
}
