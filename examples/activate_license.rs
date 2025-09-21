use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let license_key = env::var("DODO_PAYMENTS_LICENSE_KEY")
        .expect("DODO_PAYMENTS_LICENSE_KEY must be set in env variables");
    let license_name = env::var("DODO_PAYMENTS_LICENSE_NAME")
        .expect("DODO_PAYMENTS_LICENSE_NAME must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .activate_license(license_key, license_name)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
