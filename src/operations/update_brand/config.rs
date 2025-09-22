use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchBrandRequest {
    pub image_id: Option<String>,
    pub name: Option<String>,
    pub statement_descriptor: Option<String>,
    pub support_email: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateBrandConfig {
    pub id: String,
    #[serde(flatten)]
    pub body: PatchBrandRequest,
}