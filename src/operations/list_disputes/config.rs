use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDisputesConfig {
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    pub dispute_status: Option<String>,
    pub dispute_stage: Option<String>,
    pub customer_id: Option<String>,
}