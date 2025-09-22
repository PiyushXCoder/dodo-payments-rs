# Unoffical Dodo Payment SDK

<br/>
<p align="center">
<img width="200" height="" alt="Image" src="./dodo.png" align="center"/>
</p>

> [!WARNING]  
> This is Unoffical SDK of [dodopayments.com](https://dodopayments.com/)

## How to use

This project is under heavy development so not all features are available.
The structures may also change rapidly.

For now the basic usage looks much like

```rs
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
- [ ] Meters
- [ ] Usage Events
- [ ] Refunds
- [ ] Disputes
- [ ] Payouts
- [ ] Brands
- [ ] Webhooks
- [ ] Miscellaneous
