use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::create_webhook::CreateWebhookConfig,
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let webhook_url = env::var("DODO_PAYMENTS_WEBHOOK_URL")
        .expect("DODO_PAYMENTS_WEBHOOK_URL must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .create_webhook(webhook_url)
        .description("My new webhook".to_string())
        .disabled(false)
        .send()
        .await;

    println!("Response: {:#?}", response);
}
