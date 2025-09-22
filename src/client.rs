use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    common::Environment,
    operations::{
        activate_license::ActivateLicenseBuilder,
        archive_meter::ArchiveMeterBuilder,
        archive_product::ArchiveProductBuilder,
        change_plan::{ChangePlanBuilder, ProrationBillingMode},
        checkout_sessions::CheckoutSessionsBuilder,
        common::structs::*,
        create_addon::CreateAddonBuilder,
        create_brand::CreateBrandBuilder,
        create_charge::CreateChargeBuilder,
        create_customer::CreateCustomerBuilder,
        create_customer_portal_session::CreateCustomerPortalSessionBuilder,
        create_customer_wallet_ledger_entry::CreateCustomerWalletLedgerEntryBuilder,
        create_customer_wallet_ledger_entry::CustomerLedgerEntryType,
        create_discount::CreateDiscountBuilder,
        create_meter::CreateMeterBuilder,
        create_product::CreateProductBuilder,
        create_refund::CreateRefundBuilder,
        create_subscription::CreateSubscriptionBuilder,
        create_webhook::CreateWebhookBuilder,
        deactivate_license::DeactivateLicenseBuilder,
        delete_discount::DeleteDiscountBuilder,
        delete_webhook::DeleteWebhookBuilder,
        get_addon::GetAddonBuilder,
        get_brand::GetBrandBuilder,
        get_customer::GetCustomerBuilder,
        get_customer_wallets::GetCustomerWalletsBuilder,
        get_dispute::GetDisputeBuilder,
        get_event::GetEventBuilder,
        get_invoice::GetInvoiceBuilder,
        get_license_key::GetLicenseKeyBuilder,
        get_license_key_instance::GetLicenseKeyInstanceBuilder,
        get_license_key_instances::GetLicenseKeyInstancesBuilder,
        get_license_keys::GetLicenseKeysBuilder,
        get_line_items::GetLineItemsBuilder,
        get_meter::GetMeterBuilder,
        get_payment_details::GetPaymentDetailsBuilder,
        get_product::GetProductBuilder,
        get_refund::GetRefundBuilder,
        get_refund_receipt::GetRefundReceiptBuilder,
        get_subscription::GetSubscriptionBuilder,
        get_subscription_usage_history::GetSubscriptionUsageHistoryBuilder,
        get_webhook::GetWebhookBuilder,
        get_webhook_headers::GetWebhookHeadersBuilder,
        get_webhook_secret::GetWebhookSecretBuilder,
        ingest_events::IngestEventsBuilder,
        list_addons::ListAddonsBuilder,
        list_brands::ListBrandsBuilder,
        list_customer_wallet_ledger_entries::ListCustomerWalletLedgerEntriesBuilder,
        list_customers::ListCustomersBuilder,
        list_discounts::ListDiscountsBuilder,
        list_disputes::ListDisputesBuilder,
        list_events::ListEventsBuilder,
        list_meters::ListMetersBuilder,
        list_payments::ListPaymentsBuilder,
        list_payouts::ListPayoutsBuilder,
        list_products::ListProductsBuilder,
        list_refunds::ListRefundsBuilder,
        list_subscriptions::ListSubscriptionsBuilder,
        list_webhooks::ListWebhooksBuilder,
        one_time_payments::OneTimePaymentBuilder,
        patch_customer::PatchCustomerBuilder,
        unarchive_meter::UnarchiveMeterBuilder,
        unarchive_product::UnarchiveProductBuilder,
        update_addon::UpdateAddonBuilder,
        update_addon_images::UpdateAddonImagesBuilder,
        update_brand::UpdateBrandBuilder,
        update_brand_images::UpdateBrandImagesBuilder,
        update_discount::UpdateDiscountBuilder,
        update_license_key::UpdateLicenseKeyBuilder,
        update_license_key_instance::UpdateLicenseKeyInstanceBuilder,
        update_product::UpdateProductBuilder,
        update_product_files::UpdateProductFilesBuilder,
        update_product_images::UpdateProductImagesBuilder,
        update_subscription::UpdateSubscriptionBuilder,
        update_webhook::UpdateWebhookBuilder,
        update_webhook_headers::UpdateWebhookHeadersBuilder,
        validate_discount::ValidateDiscountBuilder,
        validate_license::ValidateLicenseBuilder,
        get_supported_countries::GetSupportedCountriesBuilder,
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

    pub fn create_customer_portal_session(
        &self,
        customer_id: String,
    ) -> CreateCustomerPortalSessionBuilder {
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

    pub fn list_customer_wallet_ledger_entries(
        &self,
        customer_id: String,
    ) -> ListCustomerWalletLedgerEntriesBuilder {
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

    pub fn update_brand(&self, brand_id: String) -> UpdateBrandBuilder {
        UpdateBrandBuilder::new(self.handle.clone(), brand_id)
    }

    pub fn update_brand_images(&self, brand_id: String) -> UpdateBrandImagesBuilder {
        UpdateBrandImagesBuilder::new(self.handle.clone(), brand_id)
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

    pub fn get_product(&self, id: String) -> GetProductBuilder {
        GetProductBuilder::new(self.handle.clone(), id)
    }

    pub fn update_product(&self, id: String) -> UpdateProductBuilder {
        UpdateProductBuilder::new(self.handle.clone(), id)
    }

    pub fn update_product_images(&self, id: String) -> UpdateProductImagesBuilder {
        UpdateProductImagesBuilder::new(self.handle.clone(), id)
    }

    pub fn archive_product(&self, id: String) -> ArchiveProductBuilder {
        ArchiveProductBuilder::new(self.handle.clone(), id)
    }

    pub fn unarchive_product(&self, id: String) -> UnarchiveProductBuilder {
        UnarchiveProductBuilder::new(self.handle.clone(), id)
    }

    pub fn update_product_files(&self, id: String, file_name: String) -> UpdateProductFilesBuilder {
        UpdateProductFilesBuilder::new(self.handle.clone(), id, file_name)
    }

    pub fn create_addon(
        &self,
        name: String,
        tax_category: TaxCategory,
        price: i32,
        currency: Currency,
    ) -> CreateAddonBuilder {
        CreateAddonBuilder::new(self.handle.clone(), name, tax_category, price, currency)
    }

    pub fn list_addons(&self) -> ListAddonsBuilder {
        ListAddonsBuilder::new(self.handle.clone())
    }

    pub fn get_addon(&self, addon_id: String) -> GetAddonBuilder {
        GetAddonBuilder::new(self.handle.clone(), addon_id)
    }

    pub fn update_addon(&self, addon_id: String) -> UpdateAddonBuilder {
        UpdateAddonBuilder::new(self.handle.clone(), addon_id)
    }

    pub fn update_addon_images(&self, addon_id: String) -> UpdateAddonImagesBuilder {
        UpdateAddonImagesBuilder::new(self.handle.clone(), addon_id)
    }

    pub fn create_meter(
        &self,
        name: String,
        event_name: String,
        aggregation: MeterAggregation,
        measurement_unit: String,
    ) -> CreateMeterBuilder {
        CreateMeterBuilder::new(
            self.handle.clone(),
            name,
            event_name,
            aggregation,
            measurement_unit,
        )
    }

    pub fn list_meters(&self) -> ListMetersBuilder {
        ListMetersBuilder::new(self.handle.clone())
    }

    pub fn get_meter(&self, meter_id: String) -> GetMeterBuilder {
        GetMeterBuilder::new(self.handle.clone(), meter_id)
    }

    pub fn archive_meter(&self, meter_id: String) -> ArchiveMeterBuilder {
        ArchiveMeterBuilder::new(self.handle.clone(), meter_id)
    }

    pub fn unarchive_meter(&self, meter_id: String) -> UnarchiveMeterBuilder {
        UnarchiveMeterBuilder::new(self.handle.clone(), meter_id)
    }

    pub fn ingest_events(&self, events: Vec<EventInput>) -> IngestEventsBuilder {
        IngestEventsBuilder::new(self.handle.clone(), events)
    }

    pub fn get_event(&self, event_id: String) -> GetEventBuilder {
        GetEventBuilder::new(self.handle.clone(), event_id)
    }

    pub fn list_events(&self) -> ListEventsBuilder {
        ListEventsBuilder::new(self.handle.clone())
    }

    pub fn list_refunds(&self) -> ListRefundsBuilder {
        ListRefundsBuilder::new(self.handle.clone())
    }

    pub fn create_refund(&self, payment_id: String) -> CreateRefundBuilder {
        CreateRefundBuilder::new(self.handle.clone(), payment_id)
    }

    pub fn create_webhook(&self, url: String) -> CreateWebhookBuilder {
        CreateWebhookBuilder::new(self.handle.clone(), url)
    }

    pub fn get_refund(&self, refund_id: String) -> GetRefundBuilder {
        GetRefundBuilder::new(self.handle.clone(), refund_id)
    }

    pub fn get_refund_receipt(&self, refund_id: String) -> GetRefundReceiptBuilder {
        GetRefundReceiptBuilder::new(self.handle.clone(), refund_id)
    }

    pub fn list_disputes(&self) -> ListDisputesBuilder {
        ListDisputesBuilder::new(self.handle.clone())
    }

    pub fn get_dispute(&self, dispute_id: String) -> GetDisputeBuilder {
        GetDisputeBuilder::new(self.handle.clone(), dispute_id)
    }

    pub fn list_payouts(&self) -> ListPayoutsBuilder {
        ListPayoutsBuilder::new(self.handle.clone())
    }

    pub fn list_brands(&self) -> ListBrandsBuilder {
        ListBrandsBuilder::new(self.handle.clone())
    }

    pub fn create_brand(&self) -> CreateBrandBuilder {
        CreateBrandBuilder::new(self.handle.clone())
    }

    pub fn get_brand(&self, brand_id: String) -> GetBrandBuilder {
        GetBrandBuilder::new(self.handle.clone(), brand_id)
    }

    pub fn list_webhooks(&self) -> ListWebhooksBuilder {
        ListWebhooksBuilder::new(self.handle.clone())
    }

    pub fn get_webhook(&self, webhook_id: String) -> GetWebhookBuilder {
        GetWebhookBuilder::new(self.handle.clone(), webhook_id)
    }

    pub fn update_webhook(&self, webhook_id: String) -> UpdateWebhookBuilder {
        UpdateWebhookBuilder::new(self.handle.clone(), webhook_id)
    }

    pub fn delete_webhook(&self, webhook_id: String) -> DeleteWebhookBuilder {
        DeleteWebhookBuilder::new(self.handle.clone(), webhook_id)
    }

    pub fn get_webhook_headers(&self, webhook_id: String) -> GetWebhookHeadersBuilder {
        GetWebhookHeadersBuilder::new(self.handle.clone(), webhook_id)
    }

    pub fn update_webhook_headers(
        &self,
        webhook_id: String,
        headers: HashMap<String, String>,
    ) -> UpdateWebhookHeadersBuilder {
        UpdateWebhookHeadersBuilder::new(self.handle.clone(), webhook_id, headers)
    }

    pub fn get_webhook_secret(&self, webhook_id: String) -> GetWebhookSecretBuilder {
        GetWebhookSecretBuilder::new(self.handle.clone(), webhook_id)
    }

    pub fn get_supported_countries(&self) -> GetSupportedCountriesBuilder {
        GetSupportedCountriesBuilder::new(self.handle.clone())
    }
}
