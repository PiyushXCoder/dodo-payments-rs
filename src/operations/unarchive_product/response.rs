use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnarchiveProductResponse {
    pub message: String,
}
