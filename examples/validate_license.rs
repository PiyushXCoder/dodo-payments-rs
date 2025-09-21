use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let license_key = env::var("DODO_PAYMENTS_LICENSE_KEY")
        .expect("DODO_PAYMENTS_LICENSE_KEY must be set in env variables");
    let license_key_instance_id = env::var("DODO_PAYMENTS_LICENSE_KEY_INSTANCE_ID").ok(); // Optional

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let mut builder = client.validate_license(license_key);

    if let Some(instance_id) = license_key_instance_id {
        builder = builder.license_key_instance_id(instance_id);
    }

    let response = builder.send().await;

    println!("Response: {:#?}", response);
}
