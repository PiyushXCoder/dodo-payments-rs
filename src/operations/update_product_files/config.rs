use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadProductFile {
    pub file_name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductFilesConfig {
    pub id: String,
    #[serde(flatten)]
    pub body: UploadProductFile,
}
