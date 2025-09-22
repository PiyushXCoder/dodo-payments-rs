use dodo_payments::{
    DodoPayments,
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::{
        CheckoutSessionCustomization, CheckoutSessionFlags, ProductItem, SubscriptionData,
    },
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let product_id = env::var("DODO_PAYMENTS_PRODUCT_ID")
        .expect("DODO_PAYMENTS_PRODUCT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .checkout_sessions(vec![ProductItem {
            product_id: product_id,
            quantity: 1,
            amount: None,
            addons: None,
        }])
        .customization(CheckoutSessionCustomization {
            show_on_demand_tag: true,
            show_order_details: true,
            theme: None,
        })
        .feature_flags(CheckoutSessionFlags {
            allow_currency_selection: true,
            allow_discount_code: true,
            allow_phone_number_collection: true,
            allow_tax_id: true,
            always_create_new_customer: false,
        })
        .subscription_data(SubscriptionData {
            on_demand: None,
            trial_period_days: None,
        })
        .send()
        .await;

    println!("Response: {:#?}", response);
}
