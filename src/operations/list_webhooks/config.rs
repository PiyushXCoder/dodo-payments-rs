use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListWebhooksConfig {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
}