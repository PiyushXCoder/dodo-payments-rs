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

    match client.list_discounts().send().await {
        Ok(response) => {
            println!("Discounts: {:#?}", response.items);
        }
        Err(e) => {
            eprintln!("Error listing discounts: {:#?}", e);
        }
    }
}
