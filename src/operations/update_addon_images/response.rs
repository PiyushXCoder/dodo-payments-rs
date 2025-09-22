use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAddonImagesResponse {
    pub image_id: String,
    pub url: String,
}