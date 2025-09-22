use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let customer_id = env::var("DODO_PAYMENTS_CUSTOMER_ID")
        .expect("DODO_PAYMENTS_CUSTOMER_ID must be set in env variables");
    let new_customer_name = env::var("DODO_PAYMENTS_NEW_CUSTOMER_NAME").ok();
    let new_phone_number = env::var("DODO_PAYMENTS_NEW_PHONE_NUMBER").ok();

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .patch_customer(customer_id)
        .name(new_customer_name)
        .phone_number(new_phone_number)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
