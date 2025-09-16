use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::Currency,
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
    metadata.insert("charge_reason".to_string(), "on_demand_service".to_string());

    let response = client
        .create_charge(subscription_id, 1000) // Charge 10.00 (e.g., cents)
        .adaptive_currency_fees_inclusive(true)
        .metadata(metadata)
        .product_currency(Currency::Usd)
        .product_description("On-demand service usage".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}
