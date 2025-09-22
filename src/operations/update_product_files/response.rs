use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadProductFileResponse {
    pub file_id: String,
    pub url: String,
}
