# Unoffical Dodo Payment SDK

<br/>
<p align="center">
<img width="200" height="" alt="Image" src="./dodo.png" align="center"/>
</p>

This is Unoffical SDK of [dodopayments.com](https://dodopayments.com/). Almost all features are implemented with a example script. A good amount of testing is required. 

## How to use

For now the basic usage looks much like.

```rust
use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::ProductItem,
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let product_id = env::var("DODO_PAYMENTS_PRODUCT_ID")
        .expect("DODO_PAYMENTS_PRODUCT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .checkout_sessions(vec![ProductItem {
            product_id: product_id,
            quantity: 1,
            amount: None,
        }])
        .send()
        .await;

    println!("Response: {:#?}", response);
}
```

## Running example

Check examples directory to understanding usage

```bash
export DODO_PAYMENTS_BEARER_TOKEN="<your token>"
export DODO_PAYMENTS_PRODUCT_ID="<your product id>"
cargo run --example checkout_sessions
```

## Roadmap

- [x] Checkout Session
- [x] Payments
- [x] Subscriptions
- [x] Discounts
- [x] Licenses
- [x] Customers
- [x] Products
- [x] Addons
- [x] Meters
- [x] Usage Events
- [x] Refunds
- [x] Disputes
- [x] Payouts
- [x] Brands
- [x] Webhooks
- [x] Miscellaneous

## Contributing

Welcome aboard! 🎉 We’re excited to have you here. If you’re adding a new operation, please make sure to also include a clear example that demonstrates its usage. We also encourage you to contribute by testing and refining existing operations to help maintain overall stability and reliability of the project. Every contribution—big or small—helps improve the ecosystem for everyone.
