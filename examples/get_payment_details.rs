use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let payment_id = env::var("DODO_PAYMENTS_PAYMENT_ID")
        .expect("DODO_PAYMENTS_PAYMENT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let payment_details = client.get_payment_details(payment_id).send().await?;

    println!("Payment Details: {:#?}", payment_details);

    Ok(())
}
