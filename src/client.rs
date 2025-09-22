use std::sync::Arc;

use crate::{
    common::Environment,
    operations::{
        activate_license::ActivateLicenseBuilder,
        change_plan::{ChangePlanBuilder, ProrationBillingMode},
        checkout_sessions::CheckoutSessionsBuilder,
        common::structs::*,
        create_charge::CreateChargeBuilder,
        create_customer::CreateCustomerBuilder,
        create_customer_portal_session::CreateCustomerPortalSessionBuilder,
        create_customer_wallet_ledger_entry::CreateCustomerWalletLedgerEntryBuilder,
        create_customer_wallet_ledger_entry::CustomerLedgerEntryType,
        list_customers::ListCustomersBuilder,
        get_customer::GetCustomerBuilder,
        get_customer_wallets::GetCustomerWalletsBuilder,
        list_customer_wallet_ledger_entries::ListCustomerWalletLedgerEntriesBuilder,
        patch_customer::PatchCustomerBuilder,
        create_discount::CreateDiscountBuilder,
        create_subscription::CreateSubscriptionBuilder,
        deactivate_license::DeactivateLicenseBuilder,
        delete_discount::DeleteDiscountBuilder,
        get_invoice::GetInvoiceBuilder,
        get_license_key::GetLicenseKeyBuilder,
        get_license_key_instance::GetLicenseKeyInstanceBuilder,
        get_license_key_instances::GetLicenseKeyInstancesBuilder,
        get_license_keys::GetLicenseKeysBuilder,
        get_line_items::GetLineItemsBuilder,
        get_payment_details::GetPaymentDetailsBuilder,
        get_subscription::GetSubscriptionBuilder,
        get_subscription_usage_history::GetSubscriptionUsageHistoryBuilder,
        list_discounts::ListDiscountsBuilder,
        list_payments::ListPaymentsBuilder,
        list_subscriptions::ListSubscriptionsBuilder,
        one_time_payments::OneTimePaymentBuilder,
        update_discount::UpdateDiscountBuilder,
        update_license_key::UpdateLicenseKeyBuilder,
        update_license_key_instance::UpdateLicenseKeyInstanceBuilder,
        update_subscription::UpdateSubscriptionBuilder,
        validate_discount::ValidateDiscountBuilder,
        validate_license::ValidateLicenseBuilder,
        list_products::ListProductsBuilder,
        create_product::CreateProductBuilder,
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

    pub fn create_customer_portal_session(&self, customer_id: String) -> CreateCustomerPortalSessionBuilder {
        CreateCustomerPortalSessionBuilder::new(self.handle.clone(), customer_id)
    }

    pub fn create_customer_wallet_ledger_entry(
        &self,
        customer_id: String,
        amount: i64,
        currency: Currency,
        entry_type: CustomerLedgerEntryType,
    ) -> CreateCustomerWalletLedgerEntryBuilder {
        CreateCustomerWalletLedgerEntryBuilder::new(
            self.handle.clone(),
            customer_id,
            amount,
            currency,
            entry_type,
        )
    }

    pub fn get_customer_wallets(&self, customer_id: String) -> GetCustomerWalletsBuilder {
        GetCustomerWalletsBuilder::new(self.handle.clone(), customer_id)
    }

    pub fn list_customer_wallet_ledger_entries(&self, customer_id: String) -> ListCustomerWalletLedgerEntriesBuilder {
        ListCustomerWalletLedgerEntriesBuilder::new(self.handle.clone(), customer_id)
    }

    pub fn create_customer(&self, email: String, name: String) -> CreateCustomerBuilder {
        CreateCustomerBuilder::new(self.handle.clone(), email, name)
    }

    pub fn list_customers(&self) -> ListCustomersBuilder {
        ListCustomersBuilder::new(self.handle.clone())
    }

    pub fn get_customer(&self, customer_id: String) -> GetCustomerBuilder {
        GetCustomerBuilder::new(self.handle.clone(), customer_id)
    }

    pub fn patch_customer(&self, customer_id: String) -> PatchCustomerBuilder {
        PatchCustomerBuilder::new(self.handle.clone(), customer_id)
    }

    pub fn list_payments(&self) -> ListPaymentsBuilder {
        ListPaymentsBuilder::new(self.handle.clone())
    }

    pub fn one_time_payments(
        &self,
        product_cart: Vec<ProductItem>,
        customer: CustomerRequest,
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

    pub fn create_discount(
        &self,
        amount: i32,
        discount_type: DiscountType,
    ) -> CreateDiscountBuilder {
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

    pub fn activate_license(&self, license_key: String, name: String) -> ActivateLicenseBuilder {
        ActivateLicenseBuilder::new(self.handle.clone(), license_key, name)
    }

    pub fn deactivate_license(
        &self,
        license_key: String,
        license_key_instance_id: String,
    ) -> DeactivateLicenseBuilder {
        DeactivateLicenseBuilder::new(self.handle.clone(), license_key, license_key_instance_id)
    }

    pub fn validate_license(&self, license_key: String) -> ValidateLicenseBuilder {
        ValidateLicenseBuilder::new(self.handle.clone(), license_key)
    }

    pub fn get_license_keys(&self) -> GetLicenseKeysBuilder {
        GetLicenseKeysBuilder::new(self.handle.clone())
    }

    pub fn get_license_key(&self, id: String) -> GetLicenseKeyBuilder {
        GetLicenseKeyBuilder::new(self.handle.clone(), id)
    }

    pub fn update_license_key(&self, id: String) -> UpdateLicenseKeyBuilder {
        UpdateLicenseKeyBuilder::new(self.handle.clone(), id)
    }

    pub fn get_license_key_instances(&self) -> GetLicenseKeyInstancesBuilder {
        GetLicenseKeyInstancesBuilder::new(self.handle.clone())
    }

    pub fn get_license_key_instance(&self, id: String) -> GetLicenseKeyInstanceBuilder {
        GetLicenseKeyInstanceBuilder::new(self.handle.clone(), id)
    }

    pub fn update_license_key_instance(
        &self,
        id: String,
        name: String,
    ) -> UpdateLicenseKeyInstanceBuilder {
        UpdateLicenseKeyInstanceBuilder::new(self.handle.clone(), id, name)
    }

    pub fn list_products(&self) -> ListProductsBuilder {
        ListProductsBuilder::new(self.handle.clone())
    }

    pub fn create_product(&self, tax_category: TaxCategory, price: Price) -> CreateProductBuilder {
        CreateProductBuilder::new(self.handle.clone(), tax_category, price)
    }

}
