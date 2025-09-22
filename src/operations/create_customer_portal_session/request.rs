pub use super::config::*;
use super::*;
use reqwest::Method;
use std::sync::Arc;

use crate::{
    client::Handle,
    operations::common::{make_reqwest, parse_response},
};

pub struct CreateCustomerPortalSession;

impl CreateCustomerPortalSession {
    pub async fn orchestrate(
        handle: Arc<Handle>,
        customer_id: String,
        config: CreateCustomerPortalSessionConfig,
    ) -> Result<CreateCustomerPortalSessionResponse, crate::errors::Error> {
        let query = serde_qs::to_string(&config).unwrap();
        let path = format!("/customers/{}/customer-portal/session?{}", customer_id, query);
        let response = make_reqwest(handle, Method::POST, &path, None).await?;
        let text = response.text().await?;
        parse_response(&text)
    }
}
