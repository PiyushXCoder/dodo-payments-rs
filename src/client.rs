use std::sync::Arc;

use crate::{
    common::Environment,
    operations::{checkout_sessions::CheckoutSessionsBuilder, list_payments::ListPaymentsBuilder},
};

pub struct Handle {
    pub(crate) config: DodoPaymentsConfig,
}

pub struct DodoPaymentsConfig {
    pub(crate) bearer_token: String,
    pub(crate) environment: Environment,
}

pub struct DodoPaymentsConfigBuilder {
    config: DodoPaymentsConfig,
}

impl DodoPaymentsConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: DodoPaymentsConfig {
                bearer_token: String::new(),
                environment: "test_mode".into(),
            },
        }
    }

    pub fn bearer_token(mut self, token: &str) -> Self {
        self.config.bearer_token = token.to_string();
        self
    }

    pub fn environment(mut self, env: &str) -> Self {
        self.config.environment = env.into();
        self
    }

    pub fn build(self) -> DodoPaymentsConfig {
        if self.config.bearer_token.is_empty() {
            panic!("Bearer token is required");
        }

        self.config
    }
}

pub struct DodoPayments {
    handle: Arc<Handle>,
}

impl DodoPayments {
    pub fn new(builder: DodoPaymentsConfigBuilder) -> Self {
        let config = builder.build();
        Self {
            handle: Arc::new(Handle { config }),
        }
    }

    pub fn checkout_sessions(&self) -> CheckoutSessionsBuilder {
        CheckoutSessionsBuilder::new(self.handle.clone())
    }

    pub fn list_payments(&self) -> ListPaymentsBuilder {
        ListPaymentsBuilder::new(self.handle.clone())
    }
}
