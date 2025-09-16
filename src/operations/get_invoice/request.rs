use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{client::Handle, operations::common::make_reqwest};

pub struct GetInvoice;

impl GetInvoice {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        payment_id: String,
    ) -> Result<GetInvoiceResponse, crate::errors::Error> {
        let path = format!("/invoices/payments/{}", payment_id);
        let response = make_reqwest(handle, Method::GET, &path, None).await?;
        let bytes = response.bytes().await?;
        Ok(GetInvoiceResponse { pdf_content: bytes.to_vec() })
    }
}
