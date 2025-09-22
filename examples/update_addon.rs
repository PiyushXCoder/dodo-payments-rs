use dodo_payments::{ DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");

    let addon_id = env::var("DODO_PAYMENTS_ADDON_ID")
        .expect("DODO_PAYMENTS_ADDON_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .update_addon(addon_id)
        .name("Updated Premium Support".to_string())
        .price(1499) // $14.99
        .description("24/7 premium customer support with priority response".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}

