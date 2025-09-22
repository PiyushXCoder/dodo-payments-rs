use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBrandConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub statement_descriptor: Option<String>,
    pub support_email: Option<String>,
    pub url: Option<String>,
}