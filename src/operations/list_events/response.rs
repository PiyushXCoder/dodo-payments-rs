use super::super::common::structs::Event;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListEventsResponse {
    pub items: Vec<Event>,
}