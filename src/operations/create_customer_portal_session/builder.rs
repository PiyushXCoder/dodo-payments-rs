use super::*;
use crate::{client::Handle, operations::create_customer_portal_session::CreateCustomerPortalSessionResponse};

use std::sync::Arc;

pub struct CreateCustomerPortalSessionBuilder {
    pub(crate) handle: Arc<Handle>,
    pub customer_id: String,
    pub config: CreateCustomerPortalSessionConfig,
}

impl CreateCustomerPortalSessionBuilder {
    pub fn new(handle: Arc<Handle>, customer_id: String) -> Self {
        Self {
            handle,
            customer_id,
            config: CreateCustomerPortalSessionConfig {
                send_email: None,
            },
        }
    }

    pub fn send_email(mut self, send_email: bool) -> Self {
        self.config.send_email = Some(send_email);
        self
    }

    pub async fn send(self) -> Result<CreateCustomerPortalSessionResponse, crate::errors::Error> {
        Ok(CreateCustomerPortalSession::orchestrate(self.handle, self.customer_id, self.config).await?)
    }
}
