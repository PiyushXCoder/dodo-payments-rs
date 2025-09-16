use super::config::*;
use super::response::*;
use super::request::*;
use crate::client::Handle;
use std::sync::Arc;

pub struct GetPaymentDetailsBuilder {
    pub config: GetPaymentDetailsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl GetPaymentDetailsBuilder {
    pub fn new(handle: Arc<Handle>, payment_id: String) -> Self {
        Self {
            handle,
            config: GetPaymentDetailsConfig {
                payment_id,
            },
        }
    }

    pub fn payment_id(mut self, payment_id: String) -> Self {
        self.config.payment_id = payment_id;
        self
    }

    pub async fn send(self) -> Result<PaymentResponse, crate::errors::Error> {
        Ok(GetPaymentDetails::orchestrate(self.handle, self.config).await?)
    }
}
