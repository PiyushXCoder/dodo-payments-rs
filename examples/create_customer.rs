use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let customer_email = env::var("DODO_PAYMENTS_CUSTOMER_EMAIL")
        .expect("DODO_PAYMENTS_CUSTOMER_EMAIL must be set in env variables");
    let customer_name = env::var("DODO_PAYMENTS_CUSTOMER_NAME")
        .expect("DODO_PAYMENTS_CUSTOMER_NAME must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .create_customer(customer_email, customer_name)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
