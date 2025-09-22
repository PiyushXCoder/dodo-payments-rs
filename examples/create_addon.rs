use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::{Currency, TaxCategory}
};
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
        .create_addon(
            "Premium Support".to_string(),
            TaxCategory::DigitalProducts,
            999, // $9.99
            Currency::Usd,
        )
        .description("24/7 premium customer support".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}