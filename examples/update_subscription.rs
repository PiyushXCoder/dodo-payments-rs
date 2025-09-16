use dodo_payments::{
    DodoPayments,
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::{BillingAddress, CountryCodeAlpha2, SubscriptionStatus},
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let subscription_id = env::var("DODO_PAYMENTS_SUBSCRIPTION_ID")
        .expect("DODO_PAYMENTS_SUBSCRIPTION_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());

    let response = client
        .update_subscription(subscription_id)
        .billing(BillingAddress {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "CA".to_string(),
            country: CountryCodeAlpha2::US,
            zipcode: "90210".to_string(),
        })
        .cancel_at_next_billing_date(false)
        .metadata(metadata)
        .status(SubscriptionStatus::Active)
        .tax_id("tax123".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}
