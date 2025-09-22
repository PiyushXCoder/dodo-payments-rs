use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;
use super::super::common::structs::EventType;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateWebhookConfig {
    #[validate(min_length = 1)]
    pub url: String,
    pub description: Option<String>,
    pub disabled: Option<bool>,
    pub filter_types: Option<Vec<EventType>>,
    pub headers: Option<HashMap<String, String>>,
    pub idempotency_key: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub rate_limit: Option<i32>,
}
