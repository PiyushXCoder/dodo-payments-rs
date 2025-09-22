use super::super::common::structs::EventInput;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngestEventsConfig {
    pub events: Vec<EventInput>,
}