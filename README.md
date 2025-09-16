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
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::checkout_sessions::ProductItem,
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let product_id =
        env::var("DODO_PAYMENTS_PRODUCT_ID").unwrap_or("pdt_ZEuI0QsYnwVA6fc3o1gcu".to_string());

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .checkout_sessions()
        .product_cart(vec![ProductItem {
            product_id: product_id,
            quantity: 1,
        }])
        .send()
        .await;

    println!("Response: {:?}", response);
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
- [ ] Subscriptions
- [ ] Discounts
- [ ] Licenses
- [ ] Customers
- [ ] Products
- [ ] Addons
- [ ] Meters
- [ ] Usage Events
- [ ] Refunds
- [ ] Disputes
- [ ] Payouts
- [ ] Brands
- [ ] Webhooks
- [ ] Miscellaneous
