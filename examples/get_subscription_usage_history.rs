use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder,
};
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
        .get_subscription_usage_history(subscription_id)
        .start_date("2023-01-01T00:00:00Z".to_string())
        .end_date("2023-12-31T23:59:59Z".to_string())
        .meter_id("meter_123".to_string())
        .page_size(10)
        .page_number(0)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
