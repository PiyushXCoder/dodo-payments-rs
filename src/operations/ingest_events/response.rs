use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngestEventsResponse {
    pub ingested_count: u32,
}