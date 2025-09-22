use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListEventsConfig {
    pub customer_id: Option<String>,
    pub event_name: Option<String>,
    pub meter_id: Option<String>,
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    pub start: Option<String>,
    pub end: Option<String>,
}