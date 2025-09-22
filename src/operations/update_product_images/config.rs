use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductImagesConfig {
    pub id: String,
    pub force_update: Option<bool>,
}
