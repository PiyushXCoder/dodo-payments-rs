use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::Currency};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let customer_id = env::var("DODO_PAYMENTS_CUSTOMER_ID")
        .expect("DODO_PAYMENTS_CUSTOMER_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .list_customer_wallet_ledger_entries(customer_id)
        .page_size(10)
        .page_number(0)
        .currency(Currency::Usd)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
