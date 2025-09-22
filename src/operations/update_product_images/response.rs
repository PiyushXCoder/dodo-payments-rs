use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductImageResponse {
    pub image_id: Option<String>,
    pub url: String,
}
