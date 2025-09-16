use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInvoiceResponse {
    #[serde(with = "serde_bytes")]
    pub pdf_content: Vec<u8>,
}
