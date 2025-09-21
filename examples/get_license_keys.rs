use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder, operations::get_license_keys::LicenseKeyStatus};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_license_keys()
        .page_size(10)
        .page_number(0)
        .status(LicenseKeyStatus::Active)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
