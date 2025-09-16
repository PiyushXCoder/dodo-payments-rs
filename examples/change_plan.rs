use dodo_payments::{
    DodoPayments,
    client::DodoPaymentsConfigBuilder,
    operations::change_plan::{AttachAddonReq, ProrationBillingMode},
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let subscription_id = env::var("DODO_PAYMENTS_SUBSCRIPTION_ID")
        .expect("DODO_PAYMENTS_SUBSCRIPTION_ID must be set in env variables");
    let product_id = env::var("DODO_PAYMENTS_PRODUCT_ID")
        .expect("DODO_PAYMENTS_PRODUCT_ID must be set in env variables");
    let quantity: i32 = env::var("DODO_PAYMENTS_QUANTITY")
        .expect("DODO_PAYMENTS_QUANTITY must be set in env variables")
        .parse()
        .expect("DODO_PAYMENTS_QUANTITY must be a valid integer");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .change_plan(
            subscription_id,
            product_id,
            ProrationBillingMode::ProratedImmediately,
            quantity,
        )
        .addons(vec![AttachAddonReq {
            addon_id: "addon_id_example".to_string(),
            quantity: 1,
        }])
        .send()
        .await;

    println!("Response: {:#?}", response);
}
