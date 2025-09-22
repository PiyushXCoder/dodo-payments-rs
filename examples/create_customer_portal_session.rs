use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let customer_id = env::var("DODO_PAYMENTS_CUSTOMER_ID")
        .expect("DODO_PAYMENTS_CUSTOMER_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .create_customer_portal_session(customer_id)
        .send_email(true)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
