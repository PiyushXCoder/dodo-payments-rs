use std::sync::Arc;

use crate::{
    common::Environment,
    operations::{
        change_plan::{ChangePlanBuilder, ProrationBillingMode},
        checkout_sessions::CheckoutSessionsBuilder,
        common::structs::{BillingAddress, ProductItem, DiscountType},
        create_charge::CreateChargeBuilder,
        create_subscription::{CreateSubscriptionBuilder, CustomerRequest},
        get_invoice::GetInvoiceBuilder,
        get_line_items::GetLineItemsBuilder,
        get_payment_details::GetPaymentDetailsBuilder,
        get_subscription::GetSubscriptionBuilder,
        get_subscription_usage_history::GetSubscriptionUsageHistoryBuilder,
        list_payments::ListPaymentsBuilder,
        list_subscriptions::ListSubscriptionsBuilder,
        one_time_payments::{self, OneTimePaymentBuilder},
        update_subscription::UpdateSubscriptionBuilder,
        list_discounts::ListDiscountsBuilder,
        create_discount::{CreateDiscountBuilder, CreateDiscountResponse},
        validate_discount::ValidateDiscountBuilder,
        update_discount::UpdateDiscountBuilder,
        delete_discount::DeleteDiscountBuilder,
    },
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

    pub fn checkout_sessions(&self, product_cart: Vec<ProductItem>) -> CheckoutSessionsBuilder {
        CheckoutSessionsBuilder::new(self.handle.clone(), product_cart)
    }

    pub fn list_payments(&self) -> ListPaymentsBuilder {
        ListPaymentsBuilder::new(self.handle.clone())
    }

    pub fn one_time_payments(
        &self,
        product_cart: Vec<ProductItem>,
        customer: one_time_payments::CustomerRequest,
        billing: BillingAddress,
    ) -> OneTimePaymentBuilder {
        OneTimePaymentBuilder::new(self.handle.clone(), product_cart, customer, billing)
    }

    pub fn get_payment_details(&self, payment_id: String) -> GetPaymentDetailsBuilder {
        GetPaymentDetailsBuilder::new(self.handle.clone(), payment_id)
    }

    pub fn get_invoice(&self, payment_id: String) -> GetInvoiceBuilder {
        GetInvoiceBuilder::new(self.handle.clone(), payment_id)
    }

    pub fn get_line_items(&self, payment_id: String) -> GetLineItemsBuilder {
        GetLineItemsBuilder::new(self.handle.clone(), payment_id)
    }

    pub fn list_subscriptions(&self) -> ListSubscriptionsBuilder {
        ListSubscriptionsBuilder::new(self.handle.clone())
    }

    pub fn create_subscription(
        &self,
        product_id: String,
        quantity: i32,
        customer: CustomerRequest,
        billing: BillingAddress,
    ) -> CreateSubscriptionBuilder {
        CreateSubscriptionBuilder::new(self.handle.clone(), product_id, quantity, customer, billing)
    }

    pub fn get_subscription(&self, subscription_id: String) -> GetSubscriptionBuilder {
        GetSubscriptionBuilder::new(self.handle.clone(), subscription_id)
    }

    pub fn change_plan(
        &self,
        subscription_id: String,
        product_id: String,
        proration_billing_mode: ProrationBillingMode,
        quantity: i32,
    ) -> ChangePlanBuilder {
        ChangePlanBuilder::new(
            self.handle.clone(),
            subscription_id,
            product_id,
            proration_billing_mode,
            quantity,
        )
    }

    pub fn update_subscription(&self, subscription_id: String) -> UpdateSubscriptionBuilder {
        UpdateSubscriptionBuilder::new(self.handle.clone(), subscription_id)
    }

    pub fn create_charge(
        &self,
        subscription_id: String,
        product_price: i32,
    ) -> CreateChargeBuilder {
        CreateChargeBuilder::new(self.handle.clone(), subscription_id, product_price)
    }

    pub fn get_subscription_usage_history(
        &self,
        subscription_id: String,
    ) -> GetSubscriptionUsageHistoryBuilder {
        GetSubscriptionUsageHistoryBuilder::new(self.handle.clone(), subscription_id)
    }

    pub fn list_discounts(&self) -> ListDiscountsBuilder {
        ListDiscountsBuilder::new(self.handle.clone())
    }

    pub fn create_discount(&self, amount: i32, discount_type: DiscountType) -> CreateDiscountBuilder {
        CreateDiscountBuilder::new(self.handle.clone(), amount, discount_type)
    }

    pub fn validate_discount(&self, discount_id: String) -> ValidateDiscountBuilder {
        ValidateDiscountBuilder::new(self.handle.clone(), discount_id)
    }

    pub fn update_discount(&self, discount_id: String) -> UpdateDiscountBuilder {
        UpdateDiscountBuilder::new(self.handle.clone(), discount_id)
    }

    pub fn delete_discount(&self, discount_id: String) -> DeleteDiscountBuilder {
        DeleteDiscountBuilder::new(self.handle.clone(), discount_id)
    }
}
