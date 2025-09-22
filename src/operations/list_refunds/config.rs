use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListRefundsConfig {
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    pub status: Option<String>,
    pub customer_id: Option<String>,
}