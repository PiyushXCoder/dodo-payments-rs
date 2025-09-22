use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let meter_id = env::var("DODO_PAYMENTS_METER_ID")
        .expect("DODO_PAYMENTS_METER_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .unarchive_meter(meter_id)
        .send()
        .await;

    match response {
        Ok(_) => println!("Meter unarchived successfully"),
        Err(e) => println!("Failed to unarchive meter: {:?}", e),
    }
}