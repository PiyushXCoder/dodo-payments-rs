use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let license_key_id = env::var("DODO_PAYMENTS_LICENSE_KEY_ID")
        .expect("DODO_PAYMENTS_LICENSE_KEY_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_license_key(license_key_id)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
