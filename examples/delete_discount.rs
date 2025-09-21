use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let discount_id = env::var("DODO_PAYMENTS_DISCOUNT_ID")
        .expect("DODO_PAYMENTS_DISCOUNT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    match client.delete_discount(discount_id).send().await {
        Ok(_) => {
            println!("Discount deleted successfully");
        }
        Err(e) => {
            eprintln!("Error deleting discount: {:#?}", e);
        }
    }
}
