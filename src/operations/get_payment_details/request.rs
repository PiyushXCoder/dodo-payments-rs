use super::config::*;
use super::response::*;
use crate::{client::Handle, operations::common::{make_reqwest, parse_response}};
use reqwest::Method;
use std::sync::Arc;

pub struct GetPaymentDetails;

impl GetPaymentDetails {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        config: GetPaymentDetailsConfig,
    ) -> Result<PaymentResponse, crate::errors::Error> {
        let uri = format!("/payments/{}", config.payment_id);
        let response = make_reqwest(handle, Method::GET, &uri, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
