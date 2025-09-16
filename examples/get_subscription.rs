use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

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

    let response = client
        .subscriptions()
        .retrieve(subscription_id)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
