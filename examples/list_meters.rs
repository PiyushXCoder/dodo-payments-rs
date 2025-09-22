use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
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
        .list_meters()
        .page_size(20)
        .page_number(0)
        .archived(false)
        .send()
        .await;

    println!("Response: {:#?}", response);
    
    if let Ok(resp) = response {
        println!("Found {} meters", resp.items.len());
        for meter in resp.items {
            println!("- {} ({})", meter.name, meter.id);
        }
    }
}