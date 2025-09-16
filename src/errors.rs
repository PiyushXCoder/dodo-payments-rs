#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization/Deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("API error: {0:?}")]
    ErrorResponse(crate::operations::common::ErrorResponse),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Missing field: {0}")]
    MissingField(String),
}
