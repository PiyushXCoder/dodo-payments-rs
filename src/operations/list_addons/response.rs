use super::super::common::structs::AddonResponse;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListAddonsResponse {
    pub items: Vec<AddonResponse>,
}