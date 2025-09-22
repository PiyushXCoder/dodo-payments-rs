use crate::operations::common::structs::WebhookDetails;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListWebhooksResponse {
    pub data: Vec<WebhookDetails>,
    pub done: bool,
    pub iterator: Option<String>,
    #[serde(rename = "prevIterator")]
    pub prev_iterator: Option<String>,
}