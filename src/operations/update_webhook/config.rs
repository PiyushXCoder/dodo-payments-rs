use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;
use super::super::common::structs::EventType;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWebhookConfig {
    #[serde(skip)]
    #[validate(min_length = 1)]
    pub webhook_id: String,
    pub description: Option<String>,
    pub disabled: Option<bool>,
    pub filter_types: Option<Vec<EventType>>,
    pub metadata: Option<HashMap<String, String>>,
    pub rate_limit: Option<i32>,
    pub url: Option<String>,
}
