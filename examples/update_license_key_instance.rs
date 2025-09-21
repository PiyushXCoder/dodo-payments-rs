use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let license_key_instance_id = env::var("DODO_PAYMENTS_LICENSE_KEY_INSTANCE_ID")
        .expect("DODO_PAYMENTS_LICENSE_KEY_INSTANCE_ID must be set in env variables");
    let new_instance_name = env::var("DODO_PAYMENTS_NEW_INSTANCE_NAME")
        .expect("DODO_PAYMENTS_NEW_INSTANCE_NAME must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .update_license_key_instance(license_key_instance_id, new_instance_name)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
