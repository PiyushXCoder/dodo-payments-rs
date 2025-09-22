use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::Currency, operations::create_customer_wallet_ledger_entry::CustomerLedgerEntryType};
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
        .create_customer_wallet_ledger_entry(
            customer_id,
            100,
            Currency::Usd,
            CustomerLedgerEntryType::Credit,
        )
        .reason("Initial credit for testing".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}
