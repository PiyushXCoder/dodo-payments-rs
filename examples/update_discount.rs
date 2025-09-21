use dodo_payments::operations::common::structs::DiscountType;
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

    match client
        .update_discount(discount_id)
        .amount(200) // Example: update amount to 200
        .discount_type(DiscountType::Percentage) // Example: update type to Percentage
        .send()
        .await
    {
        Ok(response) => {
            println!("Updated Discount: {:#?}", response);
        }
        Err(e) => {
            eprintln!("Error updating discount: {:#?}", e);
        }
    }
}
